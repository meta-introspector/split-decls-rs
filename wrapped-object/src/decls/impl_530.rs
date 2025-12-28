macro_rules! deps {
    () => {
        Endian!();
        BuildVersionCommand!();
        LoadCommandIterator!();
        ReadRef!();
        MachOFile!();
        MachHeader!();
        Segment!();
        DyldCacheImage!();
        SectionIndex!();
        MachOSectionInternal!();
        MachOSegmentInternal!();
        Result!();
        SymbolTable!();
    };
}

macro_rules! impl_530 {
    () => {
        deps!();
        impl < 'data , Mach , R > MachOFile < 'data , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { # [doc = " Parse the raw Mach-O file data."] pub fn parse (data : R) -> Result < Self > { let header = Mach :: parse (data , 0) ? ; let endian = header . endian () ? ; let mut segments = Vec :: new () ; let mut sections = Vec :: new () ; let mut symbols = SymbolTable :: default () ; if let Ok (mut commands) = header . load_commands (endian , data , 0) { while let Ok (Some (command)) = commands . next () { if let Some ((segment , section_data)) = Mach :: Segment :: from_command (command) ? { segments . push (MachOSegmentInternal { segment , data }) ; for section in segment . sections (endian , section_data) ? { let index = SectionIndex (sections . len () + 1) ; sections . push (MachOSectionInternal :: parse (index , section , data)) ; } } else if let Some (symtab) = command . symtab () ? { symbols = symtab . symbols (endian , data) ? ; } } } Ok (MachOFile { endian , data , header_offset : 0 , header , segments , sections , symbols , }) } # [doc = " Parse the Mach-O file for the given image from the dyld shared cache."] # [doc = " This will read different sections from different subcaches, if necessary."] pub fn parse_dyld_cache_image < 'cache , E : Endian > (image : & DyldCacheImage < 'data , 'cache , E , R > ,) -> Result < Self > { let (data , header_offset) = image . image_data_and_offset () ? ; let header = Mach :: parse (data , header_offset) ? ; let endian = header . endian () ? ; let mut segments = Vec :: new () ; let mut sections = Vec :: new () ; let mut linkedit_data : Option < R > = None ; let mut symtab = None ; if let Ok (mut commands) = header . load_commands (endian , data , header_offset) { while let Ok (Some (command)) = commands . next () { if let Some ((segment , section_data)) = Mach :: Segment :: from_command (command) ? { let addr = segment . vmaddr (endian) . into () ; let (data , _offset) = image . cache . data_and_offset_for_address (addr) . read_error ("Could not find segment data in dyld shared cache") ? ; if segment . name () == macho :: SEG_LINKEDIT . as_bytes () { linkedit_data = Some (data) ; } segments . push (MachOSegmentInternal { segment , data }) ; for section in segment . sections (endian , section_data) ? { let index = SectionIndex (sections . len () + 1) ; sections . push (MachOSectionInternal :: parse (index , section , data)) ; } } else if let Some (st) = command . symtab () ? { symtab = Some (st) ; } } } let symbols = match (symtab , linkedit_data) { (Some (symtab) , Some (linkedit_data)) => symtab . symbols (endian , linkedit_data) ? , _ => SymbolTable :: default () , } ; Ok (MachOFile { endian , data , header_offset , header , segments , sections , symbols , }) } # [doc = " Return the section at the given index."] # [inline] pub (super) fn section_internal (& self , index : SectionIndex ,) -> Result < & MachOSectionInternal < 'data , Mach , R > > { index . 0 . checked_sub (1) . and_then (| index | self . sections . get (index)) . read_error ("Invalid Mach-O section index") } # [doc = " Returns the endianness."] pub fn endian (& self) -> Mach :: Endian { self . endian } # [doc = " Returns the raw data."] pub fn data (& self) -> R { self . data } # [doc = " Returns the raw Mach-O file header."] # [deprecated (note = "Use `macho_header` instead")] pub fn raw_header (& self) -> & 'data Mach { self . header } # [doc = " Get the raw Mach-O file header."] pub fn macho_header (& self) -> & 'data Mach { self . header } # [doc = " Get the Mach-O load commands."] pub fn macho_load_commands (& self) -> Result < LoadCommandIterator < 'data , Mach :: Endian > > { self . header . load_commands (self . endian , self . data , self . header_offset) } # [doc = " Get the Mach-O symbol table."] # [doc = ""] # [doc = " Returns an empty symbol table if the file has no symbol table."] pub fn macho_symbol_table (& self) -> & SymbolTable < 'data , Mach , R > { & self . symbols } # [doc = " Return the `LC_BUILD_VERSION` load command if present."] pub fn build_version (& self) -> Result < Option < & 'data macho :: BuildVersionCommand < Mach :: Endian > > > { let mut commands = self . header . load_commands (self . endian , self . data , self . header_offset) ? ; while let Some (command) = commands . next () ? { if let Some (build_version) = command . build_version () ? { return Ok (Some (build_version)) ; } } Ok (None) } }
    };
}

impl_530!();