macro_rules! deps {
    () => {
        ImportThunkList!();
        DelayLoadImportTable!();
        ImageThunkData!();
        Import!();
        U16Bytes!();
        ImageNtHeaders!();
        Name!();
        Bytes!();
        DelayLoadDescriptorIterator!();
        Result!();
    };
}

macro_rules! impl_706 {
    () => {
        deps!();
        impl < 'data > DelayLoadImportTable < 'data > { # [doc = " Create a new delay load import table parser."] # [doc = ""] # [doc = " The import descriptors start at `import_address`."] # [doc = " This table works in the same way the import table does: descriptors will be"] # [doc = " parsed until a null entry."] # [doc = ""] # [doc = " `section_data` should be from the section containing `import_address`, and"] # [doc = " `section_address` should be the address of that section. Pointers within the"] # [doc = " descriptors and thunks may point to anywhere within the section data."] pub fn new (section_data : & 'data [u8] , section_address : u32 , import_address : u32) -> Self { DelayLoadImportTable { section_data : Bytes (section_data) , section_address , import_address , } } # [doc = " Return an iterator for the import descriptors."] pub fn descriptors (& self) -> Result < DelayLoadDescriptorIterator < 'data > > { let offset = self . import_address . wrapping_sub (self . section_address) ; let mut data = self . section_data ; data . skip (offset as usize) . read_error ("Invalid PE delay-load import descriptor address") ? ; Ok (DelayLoadDescriptorIterator { data , null : false }) } # [doc = " Return a library name given its address."] # [doc = ""] # [doc = " This address may be from [`pe::ImageDelayloadDescriptor::dll_name_rva`]."] pub fn name (& self , address : u32) -> Result < & 'data [u8] > { self . section_data . read_string_at (address . wrapping_sub (self . section_address) as usize) . read_error ("Invalid PE import descriptor name") } # [doc = " Return a list of thunks given its address."] # [doc = ""] # [doc = " This address may be from the INT, i.e. from"] # [doc = " [`pe::ImageDelayloadDescriptor::import_name_table_rva`]."] # [doc = ""] # [doc = " Please note that others RVA values from [`pe::ImageDelayloadDescriptor`] are used"] # [doc = " by the delay loader at runtime to store values, and thus do not point inside the same"] # [doc = " section as the INT. Calling this function on those addresses will fail."] pub fn thunks (& self , address : u32) -> Result < ImportThunkList < 'data > > { let offset = address . wrapping_sub (self . section_address) ; let mut data = self . section_data ; data . skip (offset as usize) . read_error ("Invalid PE delay load import thunk table address") ? ; Ok (ImportThunkList { data }) } # [doc = " Parse a thunk."] pub fn import < Pe : ImageNtHeaders > (& self , thunk : Pe :: ImageThunkData) -> Result < Import < 'data > > { if thunk . is_ordinal () { Ok (Import :: Ordinal (thunk . ordinal ())) } else { let (hint , name) = self . hint_name (thunk . address ()) ? ; Ok (Import :: Name (hint , name)) } } # [doc = " Return the hint and name at the given address."] # [doc = ""] # [doc = " This address may be from [`pe::ImageThunkData32`] or [`pe::ImageThunkData64`]."] # [doc = ""] # [doc = " The hint is an index into the export name pointer table in the target library."] pub fn hint_name (& self , address : u32) -> Result < (u16 , & 'data [u8]) > { let offset = address . wrapping_sub (self . section_address) ; let mut data = self . section_data ; data . skip (offset as usize) . read_error ("Invalid PE delay load import thunk address") ? ; let hint = data . read :: < U16Bytes < LE > > () . read_error ("Missing PE delay load import thunk hint") ? . get (LE) ; let name = data . read_string () . read_error ("Missing PE delay load import thunk name") ? ; Ok ((hint , name)) } }
    };
}

impl_706!();