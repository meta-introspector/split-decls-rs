mkuse!{use super :: Either ;}
mkuse!{use super :: mystd :: ffi :: OsStr ;}
mkuse!{use super :: mystd :: fs ;}
mkuse!{use super :: mystd :: os :: unix :: ffi :: OsStrExt ;}
mkuse!{use super :: mystd :: path :: { Path , PathBuf } ;}
mkuse!{use super :: { Context , Endian , EndianSlice , Mapping , Stash , gimli } ;}
mkuse!{use alloc :: string :: String ;}
mkuse!{use alloc :: sync :: Arc ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: convert :: { TryFrom , TryInto } ;}
mkuse!{use core :: str ;}
mkuse!{# [cfg (feature = "ruzstd")] use object :: elf :: ELFCOMPRESS_ZSTD ;}
mkuse!{use object :: elf :: { ELF_NOTE_GNU , ELFCOMPRESS_ZLIB , NT_GNU_BUILD_ID , SHF_COMPRESSED } ;}
mkuse!{use object :: read :: StringTable ;}
mkuse!{use object :: read :: elf :: { CompressionHeader , FileHeader , SectionHeader , SectionTable , Sym } ;}
mkuse!{use object :: { BigEndian , Bytes , NativeEndian } ;}
mkitem!{# [cfg (target_pointer_width = "32")] type Elf = object :: elf :: FileHeader32 < NativeEndian > ;}
mkitem!{# [cfg (target_pointer_width = "64")] type Elf = object :: elf :: FileHeader64 < NativeEndian > ;}
mkitem!{mkimpl!{impl Mapping { pub fn new (path : & Path) -> Option < Mapping > { let map = super :: mmap (path) ? ; Mapping :: mk_or_other (map , | map , stash | { let object = Object :: parse (map) ? ; if let Some (path_debug) = object . build_id () . and_then (locate_build_id) { if let Some (mapping) = Mapping :: new_debug (path , path_debug , None) { return Some (Either :: A (mapping)) ; } } if let Some ((path_debug , crc)) = object . gnu_debuglink_path (path) { if let Some (mapping) = Mapping :: new_debug (path , path_debug , Some (crc)) { return Some (Either :: A (mapping)) ; } } let dwp = Mapping :: load_dwarf_package (path , stash) ; Context :: new (stash , object , None , dwp) . map (Either :: B) }) } # [doc = " On Android, shared objects can be loaded directly from a ZIP archive"] # [doc = " (see: [`super::Library::zip_offset`])."] # [doc = ""] # [doc = " If `zip_offset` is not None, we interpret the `path` as an"] # [doc = " \"embedded\" library path, and the value of `zip_offset` tells us where"] # [doc = " in the ZIP archive the library data starts."] # [doc = ""] # [doc = " We expect `zip_offset` to be page-aligned because the dynamic linker"] # [doc = " requires this. Otherwise, loading the embedded library will fail."] # [doc = ""] # [doc = " If we fail to load an embedded library for any reason, we fallback to"] # [doc = " interpreting the path as a literal file on disk (same as calling [`Self::new`])."] # [cfg (target_os = "android")] pub fn new_android (path : & Path , zip_offset : Option < u64 >) -> Option < Mapping > { fn map_embedded_library (path : & Path , zip_offset : u64) -> Option < Mapping > { let zip_path = Path :: new (super :: extract_zip_path_android (path . as_os_str ()) ?) ; let file = fs :: File :: open (zip_path) . ok () ? ; let len = file . metadata () . ok () ? . len () ; let map = unsafe { super :: mmap :: Mmap :: map (& file , usize :: try_from (len - zip_offset) . ok () ? , zip_offset) } ? ; Mapping :: mk (map , | map , stash | { Context :: new (stash , Object :: parse (& map) ? , None , None) }) } if let Some (zip_offset) = zip_offset { map_embedded_library (path , zip_offset) . or_else (| | Self :: new (path)) } else { Self :: new (path) } } # [doc = " Load debuginfo from an external debug file."] fn new_debug (original_path : & Path , path : PathBuf , crc : Option < u32 >) -> Option < Mapping > { let map = super :: mmap (& path) ? ; Mapping :: mk (map , | map , stash | { let object = Object :: parse (map) ? ; if let Some (_crc) = crc { } let mut sup = None ; if let Some ((path_sup , build_id_sup)) = object . gnu_debugaltlink_path (& path) { if let Some (map_sup) = super :: mmap (& path_sup) { let map_sup = stash . cache_mmap (map_sup) ; if let Some (sup_) = Object :: parse (map_sup) { if sup_ . build_id () == Some (build_id_sup) { sup = Some (sup_) ; } } } } let dwp = Mapping :: load_dwarf_package (original_path , stash) ; Context :: new (stash , object , sup , dwp) }) } # [doc = " Try to locate a DWARF package file."] fn load_dwarf_package < 'data > (path : & Path , stash : & 'data Stash) -> Option < Object < 'data > > { let mut path_dwp = path . to_path_buf () ; let dwp_extension = path . extension () . map (| previous_extension | { let mut previous_extension = previous_extension . to_os_string () ; previous_extension . push (".dwp") ; previous_extension }) . unwrap_or_else (| | "dwp" . into ()) ; path_dwp . set_extension (dwp_extension) ; if let Some (map_dwp) = super :: mmap (& path_dwp) { let map_dwp = stash . cache_mmap (map_dwp) ; if let Some (dwp_) = Object :: parse (map_dwp) { return Some (dwp_) ; } } None } }}}
mkitem!{mkstruct!{struct ParsedSym { address : u64 , size : u64 , name : u32 , }}}
mkitem!{mkstruct!{pub struct Object < 'a > { # [doc = " Zero-sized type representing the native endianness."] # [doc = ""] # [doc = " We could use a literal instead, but this helps ensure correctness."] endian : NativeEndian , # [doc = " The entire file data."] data : & 'a [u8] , sections : SectionTable < 'a , Elf > , strings : StringTable < 'a > , # [doc = " List of pre-parsed and sorted symbols by base address."] syms : Vec < ParsedSym > , }}}
mkitem!{mkimpl!{impl < 'a > Object < 'a > { fn parse (data : & 'a [u8]) -> Option < Object < 'a > > { let elf = Elf :: parse (data) . ok () ? ; let endian = elf . endian () . ok () ? ; let sections = elf . sections (endian , data) . ok () ? ; let mut syms = sections . symbols (endian , data , object :: elf :: SHT_SYMTAB) . ok () ? ; if syms . is_empty () { syms = sections . symbols (endian , data , object :: elf :: SHT_DYNSYM) . ok () ? ; } let strings = syms . strings () ; let mut syms = syms . iter () . filter (| sym | { let st_type = sym . st_type () ; st_type == object :: elf :: STT_FUNC || st_type == object :: elf :: STT_OBJECT }) . filter (| sym | sym . st_shndx (endian) != object :: elf :: SHN_UNDEF) . map (| sym | { let address = sym . st_value (endian) . into () ; let size = sym . st_size (endian) . into () ; let name = sym . st_name (endian) ; ParsedSym { address , size , name , } }) . collect :: < Vec < _ > > () ; syms . sort_unstable_by_key (| s | s . address) ; Some (Object { endian , data , sections , strings , syms , }) } pub fn section (& self , stash : & 'a Stash , name : & str) -> Option < & 'a [u8] > { if let Some (section) = self . section_header (name) { let mut data = Bytes (section . data (self . endian , self . data) . ok () ?) ; let flags : u64 = section . sh_flags (self . endian) . into () ; if (flags & u64 :: from (SHF_COMPRESSED)) == 0 { return Some (data . 0) ; } let header = data . read :: < < Elf as FileHeader > :: CompressionHeader > () . ok () ? ; match header . ch_type (self . endian) { ELFCOMPRESS_ZLIB => { let size = usize :: try_from (header . ch_size (self . endian)) . ok () ? ; let buf = stash . allocate (size) ; decompress_zlib (data . 0 , buf) ? ; return Some (buf) ; } # [cfg (feature = "ruzstd")] ELFCOMPRESS_ZSTD => { let size = usize :: try_from (header . ch_size (self . endian)) . ok () ? ; let buf = stash . allocate (size) ; decompress_zstd (data . 0 , buf) ? ; return Some (buf) ; } _ => return None , } } if ! name . starts_with (".debug_") { return None ; } let debug_name = name [7 ..] . as_bytes () ; let compressed_section = self . sections . iter () . filter_map (| header | { let name = self . sections . section_name (self . endian , header) . ok () ? ; if name . starts_with (b".zdebug_") && & name [8 ..] == debug_name { Some (header) } else { None } }) . next () ? ; let mut data = Bytes (compressed_section . data (self . endian , self . data) . ok () ?) ; if data . read_bytes (8) . ok () ? . 0 != b"ZLIB\0\0\0\0" { return None ; } let size = usize :: try_from (data . read :: < object :: U32Bytes < _ > > () . ok () ? . get (BigEndian)) . ok () ? ; let buf = stash . allocate (size) ; decompress_zlib (data . 0 , buf) ? ; Some (buf) } fn section_header (& self , name : & str) -> Option < & < Elf as FileHeader > :: SectionHeader > { self . sections . section_by_name (self . endian , name . as_bytes ()) . map (| (_index , section) | section) } pub fn search_symtab (& self , addr : u64) -> Option < & [u8] > { let i = match self . syms . binary_search_by_key (& addr , | sym | sym . address) { Ok (i) => i , Err (i) => i . checked_sub (1) ? , } ; let sym = self . syms . get (i) ? ; if sym . address <= addr && addr <= sym . address + sym . size { self . strings . get (sym . name) . ok () } else { None } } pub (super) fn search_object_map (& self , _addr : u64) -> Option < (& Context < '_ > , u64) > { None } fn build_id (& self) -> Option < & 'a [u8] > { for section in self . sections . iter () { if let Ok (Some (mut notes)) = section . notes (self . endian , self . data) { while let Ok (Some (note)) = notes . next () { if note . name () == ELF_NOTE_GNU && note . n_type (self . endian) == NT_GNU_BUILD_ID { return Some (note . desc ()) ; } } } } None } fn gnu_debuglink_path (& self , path : & Path) -> Option < (PathBuf , u32) > { let section = self . section_header (".gnu_debuglink") ? ; let data = section . data (self . endian , self . data) . ok () ? ; let len = data . iter () . position (| x | * x == 0) ? ; let filename = OsStr :: from_bytes (& data [.. len]) ; let offset = (len + 1 + 3) & ! 3 ; let crc_bytes = data . get (offset .. offset + 4) . and_then (| bytes | bytes . try_into () . ok ()) ? ; let crc = u32 :: from_ne_bytes (crc_bytes) ; let path_debug = locate_debuglink (path , filename) ? ; Some ((path_debug , crc)) } fn gnu_debugaltlink_path (& self , path : & Path) -> Option < (PathBuf , & 'a [u8]) > { let section = self . section_header (".gnu_debugaltlink") ? ; let data = section . data (self . endian , self . data) . ok () ? ; let len = data . iter () . position (| x | * x == 0) ? ; let filename = OsStr :: from_bytes (& data [.. len]) ; let build_id = & data [len + 1 ..] ; let path_sup = locate_debugaltlink (path , filename , build_id) ? ; Some ((path_sup , build_id)) } }}}

macro_rules! decompress_zlib_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decompress_zlib in module {}", module_path!());
    };
}

mkfn!{
    decompress_zlib_introspect!();
    fn decompress_zlib (input : & [u8] , output : & mut [u8]) -> Option < () > { use miniz_oxide :: inflate :: TINFLStatus ; use miniz_oxide :: inflate :: core :: inflate_flags :: { TINFL_FLAG_PARSE_ZLIB_HEADER , TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF , } ; use miniz_oxide :: inflate :: core :: { DecompressorOxide , decompress } ; let (status , in_read , out_read) = decompress (& mut DecompressorOxide :: new () , input , output , 0 , TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF | TINFL_FLAG_PARSE_ZLIB_HEADER ,) ; if status == TINFLStatus :: Done && in_read == input . len () && out_read == output . len () { Some (()) } else { None } }
}

macro_rules! decompress_zstd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decompress_zstd in module {}", module_path!());
    };
}

mkfn!{
    decompress_zstd_introspect!();
    # [cfg (feature = "ruzstd")] fn decompress_zstd (mut input : & [u8] , mut output : & mut [u8]) -> Option < () > { use ruzstd :: decoding :: errors :: { FrameDecoderError , ReadFrameHeaderError } ; use ruzstd :: io :: Read ; while ! input . is_empty () { let mut decoder = match ruzstd :: decoding :: StreamingDecoder :: new (& mut input) { Ok (decoder) => decoder , Err (FrameDecoderError :: ReadFrameHeaderError (ReadFrameHeaderError :: SkipFrame { length , .. })) => { input = & input . get (length as usize ..) ? ; continue ; } Err (_) => return None , } ; loop { let bytes_written = decoder . read (output) . ok () ? ; if bytes_written == 0 { break ; } output = & mut output [bytes_written ..] ; } } if ! output . is_empty () { return None ; } Some (()) }
}
mkitem!{const DEBUG_PATH : & str = "/usr/lib/debug" ;}

macro_rules! debug_path_exists_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function debug_path_exists in module {}", module_path!());
    };
}

mkfn!{
    debug_path_exists_introspect!();
    fn debug_path_exists () -> bool { cfg_if :: cfg_if ! { if # [cfg (any (target_os = "freebsd" , target_os = "hurd" , target_os = "linux"))] { use core :: sync :: atomic :: { AtomicU8 , Ordering } ; static DEBUG_PATH_EXISTS : AtomicU8 = AtomicU8 :: new (0) ; let mut exists = DEBUG_PATH_EXISTS . load (Ordering :: Relaxed) ; if exists == 0 { exists = if Path :: new (DEBUG_PATH) . is_dir () { 1 } else { 2 } ; DEBUG_PATH_EXISTS . store (exists , Ordering :: Relaxed) ; } exists == 1 } else { false } } }
}

macro_rules! locate_build_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function locate_build_id in module {}", module_path!());
    };
}

mkfn!{
    locate_build_id_introspect!();
    # [doc = " Locate a debug file based on its build ID."] # [doc = ""] # [doc = " The format of build id paths is documented at:"] # [doc = " https://sourceware.org/gdb/onlinedocs/gdb/Separate-Debug-Files.html"] fn locate_build_id (build_id : & [u8]) -> Option < PathBuf > { const BUILD_ID_PATH : & str = "/usr/lib/debug/.build-id/" ; const BUILD_ID_SUFFIX : & str = ".debug" ; if build_id . len () < 2 { return None ; } if ! debug_path_exists () { return None ; } let mut path = String :: with_capacity (BUILD_ID_PATH . len () + BUILD_ID_SUFFIX . len () + build_id . len () * 2 + 1) ; path . push_str (BUILD_ID_PATH) ; path . push (char :: from_digit ((build_id [0] >> 4) as u32 , 16) ?) ; path . push (char :: from_digit ((build_id [0] & 0xf) as u32 , 16) ?) ; path . push ('/') ; for byte in & build_id [1 ..] { path . push (char :: from_digit ((byte >> 4) as u32 , 16) ?) ; path . push (char :: from_digit ((byte & 0xf) as u32 , 16) ?) ; } path . push_str (BUILD_ID_SUFFIX) ; Some (PathBuf :: from (path)) }
}

macro_rules! locate_debuglink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function locate_debuglink in module {}", module_path!());
    };
}

mkfn!{
    locate_debuglink_introspect!();
    # [doc = " Locate a file specified in a `.gnu_debuglink` section."] # [doc = ""] # [doc = " `path` is the file containing the section."] # [doc = " `filename` is from the contents of the section."] # [doc = ""] # [doc = " Search order is based on gdb, documented at:"] # [doc = " https://sourceware.org/gdb/onlinedocs/gdb/Separate-Debug-Files.html"] # [doc = ""] # [doc = " gdb also allows the user to customize the debug search path, but we don't."] # [doc = ""] # [doc = " gdb also supports debuginfod, but we don't yet."] fn locate_debuglink (path : & Path , filename : & OsStr) -> Option < PathBuf > { let path = fs :: canonicalize (path) . ok () ? ; let parent = path . parent () ? ; let mut f = PathBuf :: with_capacity (DEBUG_PATH . len () + parent . as_os_str () . len () + filename . len () + 2) ; let filename = Path :: new (filename) ; f . push (parent) ; f . push (filename) ; if f != path && f . is_file () { return Some (f) ; } f . clear () ; f . push (parent) ; f . push (".debug") ; f . push (filename) ; if f . is_file () { return Some (f) ; } if debug_path_exists () { f . clear () ; f . push (DEBUG_PATH) ; f . push (parent . strip_prefix ("/") . unwrap ()) ; f . push (filename) ; if f . is_file () { return Some (f) ; } } None }
}

macro_rules! locate_debugaltlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function locate_debugaltlink in module {}", module_path!());
    };
}

mkfn!{
    locate_debugaltlink_introspect!();
    # [doc = " Locate a file specified in a `.gnu_debugaltlink` section."] # [doc = ""] # [doc = " `path` is the file containing the section."] # [doc = " `filename` and `build_id` are the contents of the section."] # [doc = ""] # [doc = " Search order is based on gdb:"] # [doc = " - filename, which is either absolute or relative to `path`"] # [doc = " - the build ID path under `BUILD_ID_PATH`"] # [doc = ""] # [doc = " gdb also allows the user to customize the debug search path, but we don't."] # [doc = ""] # [doc = " gdb also supports debuginfod, but we don't yet."] fn locate_debugaltlink (path : & Path , filename : & OsStr , build_id : & [u8]) -> Option < PathBuf > { let filename = Path :: new (filename) ; if filename . is_absolute () { if filename . is_file () { return Some (filename . into ()) ; } } else { let path = fs :: canonicalize (path) . ok () ? ; let parent = path . parent () ? ; let mut f = PathBuf :: from (parent) ; f . push (filename) ; if f . is_file () { return Some (f) ; } } locate_build_id (build_id) }
}

macro_rules! handle_split_dwarf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_split_dwarf in module {}", module_path!());
    };
}

mkfn!{
    handle_split_dwarf_introspect!();
    pub (super) fn handle_split_dwarf < 'data > (package : Option < & gimli :: DwarfPackage < EndianSlice < 'data , Endian > > > , stash : & 'data Stash , load : addr2line :: SplitDwarfLoad < EndianSlice < 'data , Endian > > ,) -> Option < Arc < gimli :: Dwarf < EndianSlice < 'data , Endian > > > > { if let Some (dwp) = package . as_ref () { if let Ok (Some (cu)) = dwp . find_cu (load . dwo_id , & load . parent) { return Some (Arc :: new (cu)) ; } } let mut path = PathBuf :: new () ; if let Some (p) = load . comp_dir . as_ref () { path . push (OsStr :: from_bytes (& p)) ; } path . push (OsStr :: from_bytes (& load . path . as_ref () ?)) ; if let Some (map_dwo) = super :: mmap (& path) { let map_dwo = stash . cache_mmap (map_dwo) ; if let Some (dwo) = Object :: parse (map_dwo) { return gimli :: Dwarf :: load (| id | -> Result < _ , () > { let data = id . dwo_name () . and_then (| name | dwo . section (stash , name)) . unwrap_or (& []) ; Ok (EndianSlice :: new (data , Endian)) }) . ok () . map (| mut dwo_dwarf | { dwo_dwarf . make_dwo (& load . parent) ; Arc :: new (dwo_dwarf) }) ; } } None }
}