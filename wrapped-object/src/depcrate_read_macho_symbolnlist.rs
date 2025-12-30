// Generated macro for Nlist (trait)
macro_rules! Depcrate_read_macho_symbolNlist {
() => {
// Module: crate::read::macho::symbol
// Provides: {"Nlist"}
// Dependencies: {}
# [doc = " A trait for generic access to [`macho::Nlist32`] and [`macho::Nlist64`]."] # [allow (missing_docs)] pub trait Nlist : Debug + Pod { type Word : Into < u64 > ; type Endian : endian :: Endian ; fn n_strx (& self , endian : Self :: Endian) -> u32 ; fn n_type (& self) -> u8 ; fn n_sect (& self) -> u8 ; fn n_desc (& self , endian : Self :: Endian) -> u16 ; fn n_value (& self , endian : Self :: Endian) -> Self :: Word ; fn name < 'data , R : ReadRef < 'data > > (& self , endian : Self :: Endian , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { strings . get (self . n_strx (endian)) . read_error ("Invalid Mach-O symbol name offset") } # [doc = " Return true if this is a STAB symbol."] # [doc = ""] # [doc = " This determines the meaning of the `n_type` field."] fn is_stab (& self) -> bool { self . n_type () & macho :: N_STAB != 0 } # [doc = " Return true if this is an undefined symbol."] fn is_undefined (& self) -> bool { let n_type = self . n_type () ; n_type & macho :: N_STAB == 0 && n_type & macho :: N_TYPE == macho :: N_UNDF } # [doc = " Return true if the symbol is a definition of a function or data object."] fn is_definition (& self) -> bool { let n_type = self . n_type () ; n_type & macho :: N_STAB == 0 && n_type & macho :: N_TYPE == macho :: N_SECT } # [doc = " Return the library ordinal."] # [doc = ""] # [doc = " This is either a 1-based index into the dylib load commands,"] # [doc = " or a special ordinal."] # [inline] fn library_ordinal (& self , endian : Self :: Endian) -> u8 { (self . n_desc (endian) >> 8) as u8 } }
};
}
