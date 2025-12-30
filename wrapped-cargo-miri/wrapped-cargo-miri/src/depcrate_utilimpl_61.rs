// Generated macro for impl_61 (impl)
macro_rules! Depcrate_utilimpl_61 {
() => {
// Module: crate::util
// Provides: {"impl_61"}
// Dependencies: {}
impl CrateRunInfo { pub fn store (& self , filename : & Path) { let file = File :: create (filename) . unwrap_or_else (| _ | show_error ! ("cannot create `{}`" , filename . display ())) ; let file = BufWriter :: new (file) ; serde_json :: ser :: to_writer (file , self) . unwrap_or_else (| _ | show_error ! ("cannot write to `{}`" , filename . display ())) ; } }
};
}
