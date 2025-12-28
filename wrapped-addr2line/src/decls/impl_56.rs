macro_rules! deps {
    () => {
        ObjectContext!();
        LoaderReader!();
        Context!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'a > ObjectContext < 'a > { fn new (object : & ObjectMapFile < 'a > , arena_data : & 'a Arena < Vec < u8 > > , arena_mmap : & 'a Arena < Mmap > ,) -> Option < Self > { let file = File :: open (convert_path (object . path ()) . ok () ?) . ok () ? ; let map = & * * arena_mmap . alloc (unsafe { Mmap :: map (& file) } . ok () ?) ; let data = if let Some (member_name) = object . member () { let archive = object :: read :: archive :: ArchiveFile :: parse (map) . ok () ? ; let member = archive . members () . find_map (| member | { let member = member . ok () ? ; if member . name () == member_name { Some (member) } else { None } }) ? ; member . data (map) . ok () ? } else { map } ; let object = object :: File :: parse (data) . ok () ? ; let endian = if object . is_little_endian () { gimli :: RunTimeEndian :: Little } else { gimli :: RunTimeEndian :: Big } ; let dwarf = gimli :: Dwarf :: load (| id | load_section (Some (id . name ()) , & object , endian , arena_data)) . ok () ? ; let ctx = Context :: from_dwarf (dwarf) . ok () ? ; let symbols = object . symbol_map () ; Some (ObjectContext { ctx , symbols }) } fn ctx (& self , symbol_name : & [u8] , probe : u64) -> Option < (& Context < LoaderReader < 'a > > , u64) > { self . symbols . symbols () . iter () . find (| symbol | symbol . name () . as_bytes () == symbol_name) . map (| symbol | (& self . ctx , probe + symbol . address ())) } }
    };
}

impl_56!()