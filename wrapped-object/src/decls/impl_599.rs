macro_rules! deps {
    () => {
        SymbolMap!();
        SymbolIndex!();
        SymbolMapEntry!();
        ObjectMapEntry!();
        SymbolTable!();
        StringTable!();
        ObjectMap!();
        Endian!();
        ObjectMapFile!();
        Result!();
        MachHeader!();
        ReadRef!();
        Nlist!();
    };
}

macro_rules! impl_599 {
    () => {
        deps!();
        impl < 'data , Mach : MachHeader , R : ReadRef < 'data > > SymbolTable < 'data , Mach , R > { # [inline] pub (super) fn new (symbols : & 'data [Mach :: Nlist] , strings : StringTable < 'data , R >) -> Self { SymbolTable { symbols , strings } } # [doc = " Return the string table used for the symbol names."] # [inline] pub fn strings (& self) -> StringTable < 'data , R > { self . strings } # [doc = " Iterate over the symbols."] # [inline] pub fn iter (& self) -> slice :: Iter < 'data , Mach :: Nlist > { self . symbols . iter () } # [doc = " Return true if the symbol table is empty."] # [inline] pub fn is_empty (& self) -> bool { self . symbols . is_empty () } # [doc = " The number of symbols."] # [inline] pub fn len (& self) -> usize { self . symbols . len () } # [doc = " Return the symbol at the given index."] pub fn symbol (& self , index : SymbolIndex) -> Result < & 'data Mach :: Nlist > { self . symbols . get (index . 0) . read_error ("Invalid Mach-O symbol index") } # [doc = " Construct a map from addresses to a user-defined map entry."] pub fn map < Entry : SymbolMapEntry , F : Fn (& 'data Mach :: Nlist) -> Option < Entry > > (& self , f : F ,) -> SymbolMap < Entry > { let mut symbols = Vec :: new () ; for nlist in self . symbols { if ! nlist . is_definition () { continue ; } if let Some (entry) = f (nlist) { symbols . push (entry) ; } } SymbolMap :: new (symbols) } # [doc = " Construct a map from addresses to symbol names and object file names."] pub fn object_map (& self , endian : Mach :: Endian) -> ObjectMap < 'data > { let mut symbols = Vec :: new () ; let mut objects = Vec :: new () ; let mut object = None ; let mut current_function = None ; for nlist in self . symbols { let n_type = nlist . n_type () ; if n_type & macho :: N_STAB == 0 { continue ; } match n_type { macho :: N_SO => { object = None ; } macho :: N_OSO => { object = None ; if let Ok (name) = nlist . name (endian , self . strings) { if ! name . is_empty () { object = Some (objects . len ()) ; let (path , member) = name . split_last () . and_then (| (last , head) | { if * last != b')' { return None ; } let index = head . iter () . position (| & x | x == b'(') ? ; let (archive , rest) = head . split_at (index) ; Some ((archive , Some (& rest [1 ..]))) }) . unwrap_or ((name , None)) ; objects . push (ObjectMapFile :: new (path , member)) ; } } } macho :: N_FUN => { if let Ok (name) = nlist . name (endian , self . strings) { if ! name . is_empty () { current_function = Some ((name , nlist . n_value (endian) . into ())) } else if let Some ((name , address)) = current_function . take () { if let Some (object) = object { symbols . push (ObjectMapEntry { address , size : nlist . n_value (endian) . into () , name , object , }) ; } } } } macho :: N_STSYM => { if let Ok (name) = nlist . name (endian , self . strings) { if let Some (object) = object { symbols . push (ObjectMapEntry { address : nlist . n_value (endian) . into () , size : 0 , name , object , }) } } } _ => { } } } ObjectMap { symbols : SymbolMap :: new (symbols) , objects , } } }
    };
}

impl_599!()