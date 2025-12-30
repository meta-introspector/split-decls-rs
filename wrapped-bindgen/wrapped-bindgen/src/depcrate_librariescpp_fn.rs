// Generated macro for cpp_fn (function)
macro_rules! Depcrate_librariescpp_fn {
() => {
// Module: crate::libraries
// Provides: {"cpp_fn"}
// Dependencies: {}
fn cpp_fn (types : & [Type]) -> Option < CppFn > { let mut functions = vec ! [] ; for ty in types { if let Type :: CppFn (ty) = ty { functions . push (ty . clone ()) ; } } for ty in & functions { let arches = ty . method . arches () ; if (arches == 0) || (arches & 1 == 1) { return Some (ty . clone ()) ; } } functions . first () . cloned () }
};
}
