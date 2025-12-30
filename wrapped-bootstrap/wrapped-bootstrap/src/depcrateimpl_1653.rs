// Generated macro for impl_1653 (impl)
macro_rules! Depcrateimpl_1653 {
() => {
// Module: crate
// Provides: {"impl_1653"}
// Dependencies: {}
impl Compiler { pub fn new (stage : u32 , host : TargetSelection) -> Self { Self { stage , host , forced_compiler : false } } pub fn forced_compiler (& mut self , forced_compiler : bool) { self . forced_compiler = forced_compiler ; } # [doc = " Returns `true` if this is a snapshot compiler for `build`'s configuration"] pub fn is_snapshot (& self , build : & Build) -> bool { self . stage == 0 && self . host == build . host_target } # [doc = " Indicates whether the compiler was forced to use a specific stage."] pub fn is_forced_compiler (& self) -> bool { self . forced_compiler } }
};
}
