// Generated macro for impl_120 (impl)
macro_rules! Depcrate_defimpl_120 {
() => {
// Module: crate::def
// Provides: {"impl_120"}
// Dependencies: {}
impl NonMacroAttrKind { pub fn descr (self) -> & 'static str { match self { NonMacroAttrKind :: Builtin (..) => "built-in attribute" , NonMacroAttrKind :: Tool => "tool attribute" , NonMacroAttrKind :: DeriveHelper | NonMacroAttrKind :: DeriveHelperCompat => { "derive helper attribute" } } } pub fn article (self) -> & 'static str { "a" } # [doc = " Users of some attributes cannot mark them as used, so they are considered always used."] pub fn is_used (self) -> bool { match self { NonMacroAttrKind :: Tool | NonMacroAttrKind :: DeriveHelper | NonMacroAttrKind :: DeriveHelperCompat => true , NonMacroAttrKind :: Builtin (..) => false , } } }
};
}
