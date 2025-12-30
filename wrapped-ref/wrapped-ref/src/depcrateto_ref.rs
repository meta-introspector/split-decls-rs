// Generated macro for to_ref (function)
macro_rules! Depcrateto_ref {
() => {
// Module: crate
// Provides: {"to_ref"}
// Dependencies: {}
# [doc = "\nWrap an [`sval::Value`] in a [`ValueRef`]\n"] pub fn to_ref < 'sval , V : sval :: Value + ? Sized > (value : & 'sval V) -> Ref < & 'sval V > { Ref :: new (value) }
};
}
