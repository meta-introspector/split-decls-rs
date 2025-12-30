// Generated macro for impl_1316 (impl)
macro_rules! Depcrate_readimpl_1316 {
() => {
// Module: crate::read
// Provides: {"impl_1316"}
// Dependencies: {}
impl Relocation { # [doc = " The operation used to calculate the result of the relocation."] # [inline] pub fn kind (& self) -> RelocationKind { self . kind } # [doc = " Information about how the result of the relocation operation is encoded in the place."] # [inline] pub fn encoding (& self) -> RelocationEncoding { self . encoding } # [doc = " The size in bits of the place of the relocation."] # [doc = ""] # [doc = " If 0, then the size is determined by the relocation kind."] # [inline] pub fn size (& self) -> u8 { self . size } # [doc = " The target of the relocation."] # [inline] pub fn target (& self) -> RelocationTarget { self . target } # [doc = " The addend to use in the relocation calculation."] # [inline] pub fn addend (& self) -> i64 { self . addend } # [doc = " Set the addend to use in the relocation calculation."] # [inline] pub fn set_addend (& mut self , addend : i64) { self . addend = addend ; } # [doc = " Returns true if there is an implicit addend stored in the data at the offset"] # [doc = " to be relocated."] # [inline] pub fn has_implicit_addend (& self) -> bool { self . implicit_addend } # [doc = " Relocation flags that are specific to each file format."] # [doc = ""] # [doc = " The values returned by `kind`, `encoding` and `size` are derived"] # [doc = " from these flags."] # [inline] pub fn flags (& self) -> RelocationFlags { self . flags } }
};
}
