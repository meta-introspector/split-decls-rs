macro_rules! deps {
    () => {
        NoteHeader32!();
        U32!();
        StandardSection!();
        BinaryFormat!();
        GnuProperty!();
        Object!();
    };
}

macro_rules! impl_959 {
    () => {
        deps!();
        impl < 'a > Object < 'a > { # [doc = " Add a property with a u32 value to the ELF \".note.gnu.property\" section."] # [doc = ""] # [doc = " Requires `feature = \"elf\"`."] pub fn add_elf_gnu_property_u32 (& mut self , property : u32 , value : u32) { if self . format != BinaryFormat :: Elf { return ; } let align = if self . elf_is_64 () { 8 } else { 4 } ; let mut data = Vec :: with_capacity (32) ; let n_name = b"GNU\0" ; data . extend_from_slice (pod :: bytes_of (& elf :: NoteHeader32 { n_namesz : U32 :: new (self . endian , n_name . len () as u32) , n_descsz : U32 :: new (self . endian , util :: align (3 * 4 , align) as u32) , n_type : U32 :: new (self . endian , elf :: NT_GNU_PROPERTY_TYPE_0) , })) ; data . extend_from_slice (n_name) ; debug_assert_eq ! (util :: align (data . len () , align) , data . len ()) ; data . extend_from_slice (pod :: bytes_of (& U32 :: new (self . endian , property))) ; data . extend_from_slice (pod :: bytes_of (& U32 :: new (self . endian , 4))) ; data . extend_from_slice (pod :: bytes_of (& U32 :: new (self . endian , value))) ; util :: write_align (& mut data , align) ; let section = self . section_id (StandardSection :: GnuProperty) ; self . append_section_data (section , & data , align as u64) ; } }
    };
}

impl_959!()