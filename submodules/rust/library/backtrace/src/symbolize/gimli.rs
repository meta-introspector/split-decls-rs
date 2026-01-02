mkuse!{use self :: gimli :: NativeEndian as Endian ;}
mkuse!{use self :: gimli :: read :: EndianSlice ;}
mkuse!{use self :: mmap :: Mmap ;}
mkuse!{use self :: stash :: Stash ;}
mkuse!{use super :: BytesOrWideString ;}
mkuse!{use super :: ResolveWhat ;}
mkuse!{use super :: SymbolName ;}
mkuse!{use addr2line :: gimli ;}
mkuse!{use core :: convert :: TryInto ;}
mkuse!{use core :: mem ;}
mkuse!{use libc :: c_void ;}
mkuse!{use mystd :: ffi :: OsString ;}
mkuse!{use mystd :: fs :: File ;}
mkuse!{use mystd :: path :: Path ;}
mkuse!{use mystd :: prelude :: v1 :: * ;}
mkmod!{mystd, { 
                getname!(mystd);
                getsrc!(mystd);
                getpath!(mystd);
                get_deps!(mystd);
                get_crates!(mystd);
                mkinclude!(mystd);
                mkuse!{pub use crate :: * ;} 
            }}
mkitem!{# [cfg (not (backtrace_in_libstd))] extern crate std as mystd ;}
mkitem!{cfg_if :: cfg_if ! { if # [cfg (windows)] { # [path = "gimli/mmap_windows.rs"] mod mmap ; } else if # [cfg (target_vendor = "apple")] { # [path = "gimli/mmap_unix.rs"] mod mmap ; } else if # [cfg (any (target_os = "android" , target_os = "freebsd" , target_os = "fuchsia" , target_os = "haiku" , target_os = "hurd" , target_os = "linux" , target_os = "openbsd" , target_os = "solaris" , target_os = "illumos" , target_os = "aix" , target_os = "cygwin" ,))] { # [path = "gimli/mmap_unix.rs"] mod mmap ; } else { # [path = "gimli/mmap_fake.rs"] mod mmap ; } }}
mkmod!{lru, { 
                getname!(lru);
                getsrc!(lru);
                getpath!(lru);
                get_deps!(lru);
                get_crates!(lru);
                mkinclude!(lru);
                 
            }}
mkmod!{stash, { 
                getname!(stash);
                getsrc!(stash);
                getpath!(stash);
                get_deps!(stash);
                get_crates!(stash);
                mkinclude!(stash);
                 
            }}
mkuse!{use lru :: Lru ;}
mkitem!{const MAPPINGS_CACHE_SIZE : usize = 4 ;}
mkitem!{mkstruct!{struct Mapping { cx : Context < 'static > , _map : Mmap , stash : Stash , }}}
mkitem!{mkenum!{enum Either < A , B > { # [allow (dead_code)] A (A) , B (B) , }}}
mkitem!{mkimpl!{impl Mapping { # [doc = " Creates a `Mapping` by ensuring that the `data` specified is used to"] # [doc = " create a `Context` and it can only borrow from that or the `Stash` of"] # [doc = " decompressed sections or auxiliary data."] fn mk < F > (data : Mmap , mk : F) -> Option < Mapping > where F : for < 'a > FnOnce (& 'a [u8] , & 'a Stash) -> Option < Context < 'a > > , { Mapping :: mk_or_other (data , move | data , stash | { let cx = mk (data , stash) ? ; Some (Either :: B (cx)) }) } # [doc = " Creates a `Mapping` from `data`, or if the closure decides to, returns a"] # [doc = " different mapping."] fn mk_or_other < F > (data : Mmap , mk : F) -> Option < Mapping > where F : for < 'a > FnOnce (& 'a [u8] , & 'a Stash) -> Option < Either < Mapping , Context < 'a > > > , { let stash = Stash :: new () ; let cx = match mk (& data , & stash) ? { Either :: A (mapping) => return Some (mapping) , Either :: B (cx) => cx , } ; Some (Mapping { cx : unsafe { core :: mem :: transmute :: < Context < '_ > , Context < 'static > > (cx) } , _map : data , stash , }) } }}}
mkitem!{mkstruct!{struct Context < 'a > { dwarf : addr2line :: Context < EndianSlice < 'a , Endian > > , object : Object < 'a > , package : Option < gimli :: DwarfPackage < EndianSlice < 'a , Endian > > > , }}}
mkitem!{mkimpl!{impl < 'data > Context < 'data > { # [cfg_attr (backtrace_in_libstd , optimize (size))] fn new (stash : & 'data Stash , object : Object < 'data > , sup : Option < Object < 'data > > , dwp : Option < Object < 'data > > ,) -> Option < Context < 'data > > { let mut sections = gimli :: Dwarf :: load (| id | -> Result < _ , () > { if cfg ! (not (target_os = "aix")) { let data = object . section (stash , id . name ()) . unwrap_or (& []) ; Ok (EndianSlice :: new (data , Endian)) } else if let Some (name) = id . xcoff_name () { let data = object . section (stash , name) . unwrap_or (& []) ; Ok (EndianSlice :: new (data , Endian)) } else { Ok (EndianSlice :: new (& [] , Endian)) } }) . ok () ? ; if let Some (sup) = sup { sections . load_sup (| id | -> Result < _ , () > { let data = sup . section (stash , id . name ()) . unwrap_or (& []) ; Ok (EndianSlice :: new (data , Endian)) }) . ok () ? ; } let dwarf = addr2line :: Context :: from_dwarf (sections) . ok () ? ; let mut package = None ; if let Some (dwp) = dwp { package = Some (gimli :: DwarfPackage :: load (| id | -> Result < _ , gimli :: Error > { let data = id . dwo_name () . and_then (| name | dwp . section (stash , name)) . unwrap_or (& []) ; Ok (EndianSlice :: new (data , Endian)) } , EndianSlice :: new (& [] , Endian) ,) . ok () ? ,) ; } Some (Context { dwarf , object , package , }) } fn find_frames (& '_ self , stash : & 'data Stash , probe : u64 ,) -> gimli :: Result < addr2line :: FrameIter < '_ , EndianSlice < 'data , Endian > > > { use addr2line :: { LookupContinuation , LookupResult } ; let mut l = self . dwarf . find_frames (probe) ; loop { let (load , continuation) = match l { LookupResult :: Output (output) => break output , LookupResult :: Load { load , continuation } => (load , continuation) , } ; l = continuation . resume (handle_split_dwarf (self . package . as_ref () , stash , load)) ; } } }}}

macro_rules! mmap_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mmap in module {}", module_path!());
    };
}

mkfn!{
    mmap_introspect!();
    fn mmap (path : & Path) -> Option < Mmap > { let file = File :: open (path) . ok () ? ; let len = file . metadata () . ok () ? . len () . try_into () . ok () ? ; unsafe { Mmap :: map (& file , len , 0) } }
}
mkitem!{cfg_if :: cfg_if ! { if # [cfg (any (windows , target_os = "cygwin"))] { mod coff ; use self :: coff :: { handle_split_dwarf , Object } ; } else if # [cfg (any (target_vendor = "apple"))] { mod macho ; use self :: macho :: { handle_split_dwarf , Object } ; } else if # [cfg (target_os = "aix")] { mod xcoff ; use self :: xcoff :: { handle_split_dwarf , Object } ; } else { mod elf ; use self :: elf :: { handle_split_dwarf , Object } ; } }}
mkitem!{cfg_if :: cfg_if ! { if # [cfg (any (windows , target_os = "cygwin"))] { mod libs_windows ; use libs_windows :: native_libraries ; } else if # [cfg (target_vendor = "apple")] { mod libs_macos ; use libs_macos :: native_libraries ; } else if # [cfg (target_os = "illumos")] { mod libs_illumos ; use libs_illumos :: native_libraries ; } else if # [cfg (all (any (target_os = "linux" , target_os = "fuchsia" , target_os = "freebsd" , target_os = "hurd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "nto" , target_os = "android" ,) , not (target_env = "uclibc") ,))] { mod libs_dl_iterate_phdr ; use libs_dl_iterate_phdr :: native_libraries ; # [path = "gimli/parse_running_mmaps_unix.rs"] mod parse_running_mmaps ; } else if # [cfg (target_env = "libnx")] { mod libs_libnx ; use libs_libnx :: native_libraries ; } else if # [cfg (target_os = "haiku")] { mod libs_haiku ; use libs_haiku :: native_libraries ; } else if # [cfg (target_os = "aix")] { mod libs_aix ; use libs_aix :: native_libraries ; } else { fn native_libraries () -> Vec < Library > { Vec :: new () } } }}
mkitem!{mkstruct!{# [derive (Default)] struct Cache { # [doc = " All known shared libraries that have been loaded."] libraries : Vec < Library > , # [doc = " Mappings cache where we retain parsed dwarf information."] # [doc = ""] # [doc = " This list has a fixed capacity for its entire lifetime which never"] # [doc = " increases. The `usize` element of each pair is an index into `libraries`"] # [doc = " above where `usize::max_value()` represents the current executable. The"] # [doc = " `Mapping` is corresponding parsed dwarf information."] # [doc = ""] # [doc = " Note that this is basically an LRU cache and we'll be shifting things"] # [doc = " around in here as we symbolize addresses."] mappings : Lru < (usize , Mapping) , MAPPINGS_CACHE_SIZE > , }}}
mkitem!{mkstruct!{struct Library { name : OsString , # [cfg (target_os = "android")] # [doc = " On Android, the dynamic linker [can map libraries directly from a"] # [doc = " ZIP archive][ndk-linker-changes] (typically an `.apk`)."] # [doc = ""] # [doc = " The linker requires that these libraries are stored uncompressed"] # [doc = " and page-aligned."] # [doc = ""] # [doc = " These \"embedded\" libraries have filepaths of the form"] # [doc = " `/path/to/my.apk!/lib/mylib.so` (where `/path/to/my.apk` is the archive"] # [doc = " and `lib/mylib.so` is the name of the library within the archive)."] # [doc = ""] # [doc = " This mechanism is present on Android since API level 23."] # [doc = ""] # [doc = " [ndk-linker-changes]: https://android.googlesource.com/platform/bionic/+/main/android-changes-for-ndk-developers.md#opening-shared-libraries-directly-from-an-apk"] zip_offset : Option < u64 > , # [cfg (target_os = "aix")] # [doc = " On AIX, the library mmapped can be a member of a big-archive file."] # [doc = " For example, with a big-archive named libfoo.a containing libbar.so,"] # [doc = " one can use `dlopen(\"libfoo.a(libbar.so)\", RTLD_MEMBER | RTLD_LAZY)`"] # [doc = " to use the `libbar.so` library. In this case, only `libbar.so` is"] # [doc = " mmapped, not the whole `libfoo.a`."] member_name : OsString , # [doc = " Segments of this library loaded into memory, and where they're loaded."] segments : Vec < LibrarySegment > , # [doc = " The \"bias\" of this library, typically where it's loaded into memory."] # [doc = " This value is added to each segment's stated address to get the actual"] # [doc = " virtual memory address that the segment is loaded into. Additionally"] # [doc = " this bias is subtracted from real virtual memory addresses to index into"] # [doc = " debuginfo and the symbol table."] bias : usize , }}}
mkitem!{mkstruct!{struct LibrarySegment { # [doc = " The stated address of this segment in the object file. This is not"] # [doc = " actually where the segment is loaded, but rather this address plus the"] # [doc = " containing library's `bias` is where to find it."] stated_virtual_memory_address : usize , # [doc = " The size of this segment in memory."] len : usize , }}}

macro_rules! create_mapping_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_mapping in module {}", module_path!());
    };
}

mkfn!{
    create_mapping_introspect!();
    fn create_mapping (lib : & Library) -> Option < Mapping > { cfg_if :: cfg_if ! { if # [cfg (target_os = "aix")] { Mapping :: new (lib . name . as_ref () , & lib . member_name) } else if # [cfg (target_os = "android")] { Mapping :: new_android (lib . name . as_ref () , lib . zip_offset) } else { Mapping :: new (lib . name . as_ref ()) } } }
}

macro_rules! extract_zip_path_android_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract_zip_path_android in module {}", module_path!());
    };
}

mkfn!{
    extract_zip_path_android_introspect!();
    # [doc = " Try to extract the archive path from an \"embedded\" library path"] # [doc = " (e.g. `/path/to/my.apk` from `/path/to/my.apk!/mylib.so`)."] # [doc = ""] # [doc = " Returns `None` if the path does not contain a `!/` separator."] # [cfg (target_os = "android")] fn extract_zip_path_android (path : & mystd :: ffi :: OsStr) -> Option < & mystd :: ffi :: OsStr > { use mystd :: os :: unix :: ffi :: OsStrExt ; path . as_bytes () . windows (2) . enumerate () . find (| (_ , chunk) | chunk == b"!/") . map (| (index , _) | mystd :: ffi :: OsStr :: from_bytes (path . as_bytes () . split_at (index) . 0)) }
}

macro_rules! clear_symbol_cache_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clear_symbol_cache in module {}", module_path!());
    };
}

mkfn!{
    clear_symbol_cache_introspect!();
    pub unsafe fn clear_symbol_cache () { unsafe { Cache :: with_global (| cache | cache . mappings . clear ()) ; } }
}
mkitem!{mkimpl!{impl Cache { fn new () -> Cache { Cache { mappings : Lru :: default () , libraries : native_libraries () , } } # [cfg_attr (backtrace_in_libstd , optimize (size))] unsafe fn with_global (f : impl FnOnce (& mut Self)) { static mut MAPPINGS_CACHE : Option < Cache > = None ; unsafe { # [allow (static_mut_refs)] f (MAPPINGS_CACHE . get_or_insert_with (Cache :: new)) } } fn avma_to_svma (& self , addr : * const u8) -> Option < (usize , * const u8) > { self . libraries . iter () . enumerate () . filter_map (| (i , lib) | { if ! lib . segments . iter () . any (| s | { let svma = s . stated_virtual_memory_address ; let start = svma . wrapping_add (lib . bias) ; let end = start . wrapping_add (s . len) ; let address = addr as usize ; start <= address && address < end }) { return None ; } let svma = (addr as usize) . wrapping_sub (lib . bias) ; Some ((i , svma as * const u8)) }) . next () } fn mapping_for_lib < 'a > (& 'a mut self , lib : usize) -> Option < (& 'a mut Context < 'a > , & 'a Stash) > { let cache_idx = self . mappings . iter () . position (| (lib_id , _) | * lib_id == lib) ; let cache_entry = if let Some (idx) = cache_idx { self . mappings . move_to_front (idx) } else { create_mapping (& self . libraries [lib]) . and_then (| mapping | self . mappings . push_front ((lib , mapping))) } ; let (_ , mapping) = cache_entry ? ; let cx : & 'a mut Context < 'static > = & mut mapping . cx ; let stash : & 'a Stash = & mapping . stash ; Some ((unsafe { mem :: transmute :: < & 'a mut Context < 'static > , & 'a mut Context < 'a > > (cx) } , stash ,)) } }}}

macro_rules! resolve_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve in module {}", module_path!());
    };
}

mkfn!{
    resolve_introspect!();
    pub unsafe fn resolve (what : ResolveWhat < '_ > , cb : & mut dyn FnMut (& super :: Symbol)) { let addr = what . address_or_ip () ; let mut call = | sym : Symbol < '_ > | { let sym = unsafe { mem :: transmute :: < Symbol < '_ > , Symbol < 'static > > (sym) } ; (cb) (& super :: Symbol { inner : sym }) ; } ; unsafe { Cache :: with_global (| cache | { let (lib , addr) = match cache . avma_to_svma (addr . cast_const () . cast :: < u8 > ()) { Some (pair) => pair , None => return , } ; let (cx , stash) = match cache . mapping_for_lib (lib) { Some ((cx , stash)) => (cx , stash) , None => return , } ; let mut any_frames = false ; if let Ok (mut frames) = cx . find_frames (stash , addr as u64) { while let Ok (Some (frame)) = frames . next () { any_frames = true ; let name = match frame . function { Some (f) => Some (f . name . slice ()) , None => cx . object . search_symtab (addr as u64) , } ; call (Symbol :: Frame { addr : addr as * mut c_void , location : frame . location , name , }) ; } } if ! any_frames { if let Some ((object_cx , object_addr)) = cx . object . search_object_map (addr as u64) { if let Ok (mut frames) = object_cx . find_frames (stash , object_addr) { while let Ok (Some (frame)) = frames . next () { any_frames = true ; call (Symbol :: Frame { addr : addr as * mut c_void , location : frame . location , name : frame . function . map (| f | f . name . slice ()) , }) ; } } } } if ! any_frames { if let Some (name) = cx . object . search_symtab (addr as u64) { call (Symbol :: Symtab { name }) ; } } }) ; } }
}
mkitem!{mkenum!{pub enum Symbol < 'a > { # [doc = " We were able to locate frame information for this symbol, and"] # [doc = " `addr2line`'s frame internally has all the nitty gritty details."] Frame { addr : * mut c_void , location : Option < addr2line :: Location < 'a > > , name : Option < & 'a [u8] > , } , # [doc = " Couldn't find debug information, but we found it in the symbol table of"] # [doc = " the elf executable."] Symtab { name : & 'a [u8] } , }}}
mkitem!{mkimpl!{impl Symbol < '_ > { pub fn name (& self) -> Option < SymbolName < '_ > > { match self { Symbol :: Frame { name , .. } => { let name = name . as_ref () ? ; Some (SymbolName :: new (name)) } Symbol :: Symtab { name , .. } => Some (SymbolName :: new (name)) , } } pub fn addr (& self) -> Option < * mut c_void > { match self { Symbol :: Frame { addr , .. } => Some (* addr) , Symbol :: Symtab { .. } => None , } } pub fn filename_raw (& self) -> Option < BytesOrWideString < '_ > > { match self { Symbol :: Frame { location , .. } => { let file = location . as_ref () ? . file ? ; Some (BytesOrWideString :: Bytes (file . as_bytes ())) } Symbol :: Symtab { .. } => None , } } pub fn filename (& self) -> Option < & Path > { match self { Symbol :: Frame { location , .. } => { let file = location . as_ref () ? . file ? ; Some (Path :: new (file)) } Symbol :: Symtab { .. } => None , } } pub fn lineno (& self) -> Option < u32 > { match self { Symbol :: Frame { location , .. } => location . as_ref () ? . line , Symbol :: Symtab { .. } => None , } } pub fn colno (& self) -> Option < u32 > { match self { Symbol :: Frame { location , .. } => location . as_ref () ? . column , Symbol :: Symtab { .. } => None , } } }}}