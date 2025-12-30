// Generated macro for impl_133 (impl)
macro_rules! Depcrate_filesimpl_133 {
() => {
// Module: crate::files
// Provides: {"impl_133"}
// Dependencies: {}
impl < FileKind : Copy , T : Clone > InFileWrapper < FileKind , & T > { pub fn cloned (& self) -> InFileWrapper < FileKind , T > { self . with_value (self . value . clone ()) } }
};
}
