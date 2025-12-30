// Generated macro for demangle (function)
macro_rules! Depcratedemangle {
() => {
// Module: crate
// Provides: {"demangle"}
// Dependencies: {}
fn demangle (module : & mut Module) { for func in module . funcs . iter_mut () { let name = match & func . name { Some (name) => name , None => continue , } ; if let Ok (sym) = rustc_demangle :: try_demangle (name) { func . name = Some (sym . to_string ()) ; } } }
};
}
