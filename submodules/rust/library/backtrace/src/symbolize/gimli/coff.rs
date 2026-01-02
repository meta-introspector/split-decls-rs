mkuse!{use super :: mystd :: path :: Path ;}
mkuse!{use super :: { Context , Endian , EndianSlice , Mapping , Stash , gimli } ;}
mkuse!{use alloc :: sync :: Arc ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: convert :: TryFrom ;}
mkuse!{use object :: LittleEndian as LE ;}
mkuse!{use object :: pe :: { ImageDosHeader , ImageSymbol } ;}
mkuse!{use object :: read :: StringTable ;}
mkuse!{use object :: read :: coff :: ImageSymbol as _ ;}
mkuse!{use object :: read :: pe :: { ImageNtHeaders , ImageOptionalHeader , SectionTable } ;}
mkitem!{# [cfg (target_pointer_width = "32")] type Pe = object :: pe :: ImageNtHeaders32 ;}
mkitem!{# [cfg (target_pointer_width = "64")] type Pe = object :: pe :: ImageNtHeaders64 ;}
mkitem!{mkimpl!{impl Mapping { pub fn new (path : & Path) -> Option < Mapping > { let map = super :: mmap (path) ? ; Mapping :: mk (map , | data , stash | { Context :: new (stash , Object :: parse (data) ? , None , None) }) } }}}
mkitem!{mkstruct!{pub struct Object < 'a > { data : & 'a [u8] , sections : SectionTable < 'a > , symbols : Vec < (usize , & 'a ImageSymbol) > , strings : StringTable < 'a > , }}}

macro_rules! get_image_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_image_base in module {}", module_path!());
    };
}

mkfn!{
    get_image_base_introspect!();
    pub fn get_image_base (data : & [u8]) -> Option < usize > { let dos_header = ImageDosHeader :: parse (data) . ok () ? ; let mut offset = dos_header . nt_headers_offset () . into () ; let (nt_headers , _) = Pe :: parse (data , & mut offset) . ok () ? ; usize :: try_from (nt_headers . optional_header () . image_base ()) . ok () }
}
mkitem!{mkimpl!{impl < 'a > Object < 'a > { fn parse (data : & 'a [u8]) -> Option < Object < 'a > > { let dos_header = ImageDosHeader :: parse (data) . ok () ? ; let mut offset = dos_header . nt_headers_offset () . into () ; let (nt_headers , _) = Pe :: parse (data , & mut offset) . ok () ? ; let sections = nt_headers . sections (data , offset) . ok () ? ; let symtab = nt_headers . symbols (data) . ok () ? ; let strings = symtab . strings () ; let image_base = usize :: try_from (nt_headers . optional_header () . image_base ()) . ok () ? ; let mut symbols = Vec :: new () ; for (_ , sym) in symtab . iter () { if sym . derived_type () != object :: pe :: IMAGE_SYM_DTYPE_FUNCTION { continue ; } let Some (section_index) = sym . section () else { continue ; } ; let addr = usize :: try_from (sym . value . get (LE)) . ok () ? ; let section = sections . section (section_index) . ok () ? ; let va = usize :: try_from (section . virtual_address . get (LE)) . ok () ? ; symbols . push ((addr + va + image_base , sym)) ; } symbols . sort_unstable_by_key (| x | x . 0) ; Some (Object { data , sections , strings , symbols , }) } pub fn section (& self , _ : & Stash , name : & str) -> Option < & 'a [u8] > { Some (self . sections . section_by_name (self . strings , name . as_bytes ()) ? . 1 . pe_data (self . data) . ok () ? ,) } pub fn search_symtab < 'b > (& 'b self , addr : u64) -> Option < & 'b [u8] > { let addr = usize :: try_from (addr) . ok () ? ; let i = match self . symbols . binary_search_by_key (& addr , | p | p . 0) { Ok (i) => i , Err (i) => i . checked_sub (1) ? , } ; self . symbols [i] . 1 . name (self . strings) . ok () } pub (super) fn search_object_map (& self , _addr : u64) -> Option < (& Context < '_ > , u64) > { None } }}}

macro_rules! handle_split_dwarf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_split_dwarf in module {}", module_path!());
    };
}

mkfn!{
    handle_split_dwarf_introspect!();
    pub (super) fn handle_split_dwarf < 'data > (_package : Option < & gimli :: DwarfPackage < EndianSlice < 'data , Endian > > > , _stash : & 'data Stash , _load : addr2line :: SplitDwarfLoad < EndianSlice < 'data , Endian > > ,) -> Option < Arc < gimli :: Dwarf < EndianSlice < 'data , Endian > > > > { None }
}