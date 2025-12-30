// Generated macro for Reference (struct)
macro_rules! Depcrate_core_build_steps_docReference {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"Reference"}
// Dependencies: {}
# [doc = " Documents the reference."] # [doc = " It has to always be done using a stage 1+ compiler, because it references in-tree"] # [doc = " compiler/stdlib concepts."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Reference { build_compiler : Compiler , target : TargetSelection , }
};
}
