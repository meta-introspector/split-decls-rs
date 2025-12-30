// Generated macro for Relocation (struct)
macro_rules! Depcrate_readRelocation {
() => {
// Module: crate::read
// Provides: {"Relocation"}
// Dependencies: {}
# [doc = " A relocation entry."] # [doc = ""] # [doc = " Returned by [`Object::dynamic_relocations`] or [`ObjectSection::relocations`]."] # [derive (Debug)] pub struct Relocation { kind : RelocationKind , encoding : RelocationEncoding , size : u8 , target : RelocationTarget , addend : i64 , implicit_addend : bool , flags : RelocationFlags , }
};
}
