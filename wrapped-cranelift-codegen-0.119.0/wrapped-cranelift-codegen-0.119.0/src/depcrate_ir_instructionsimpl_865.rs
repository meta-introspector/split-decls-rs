// Generated macro for impl_865 (impl)
macro_rules! Depcrate_ir_instructionsimpl_865 {
() => {
// Module: crate::ir::instructions
// Provides: {"impl_865"}
// Dependencies: {}
impl VariableArgs { # [doc = " Create an empty argument list."] pub fn new () -> Self { Self (Vec :: new ()) } # [doc = " Add an argument to the end."] pub fn push (& mut self , v : Value) { self . 0 . push (v) } # [doc = " Check if the list is empty."] pub fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Convert this to a value list in `pool` with `fixed` prepended."] pub fn into_value_list (self , fixed : & [Value] , pool : & mut ValueListPool) -> ValueList { let mut vlist = ValueList :: default () ; vlist . extend (fixed . iter () . cloned () , pool) ; vlist . extend (self . 0 , pool) ; vlist } }
};
}
