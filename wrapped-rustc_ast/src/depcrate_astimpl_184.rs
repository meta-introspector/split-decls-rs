// Generated macro for impl_184 (impl)
macro_rules! Depcrate_astimpl_184 {
() => {
// Module: crate::ast
// Provides: {"impl_184"}
// Dependencies: {}
impl FnDecl { pub fn has_self (& self) -> bool { self . inputs . get (0) . is_some_and (Param :: is_self) } pub fn c_variadic (& self) -> bool { self . inputs . last () . is_some_and (| arg | matches ! (arg . ty . kind , TyKind :: CVarArgs)) } }
};
}
