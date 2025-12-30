// Generated macro for Path (struct)
macro_rules! Depcrate_deriving_generic_tyPath {
() => {
// Module: crate::deriving::generic::ty
// Provides: {"Path"}
// Dependencies: {}
# [doc = " A path, e.g., `::std::option::Option::<i32>` (global). Has support"] # [doc = " for type parameters."] # [derive (Clone)] pub (crate) struct Path { path : Vec < Symbol > , params : Vec < Box < Ty > > , kind : PathKind , }
};
}
