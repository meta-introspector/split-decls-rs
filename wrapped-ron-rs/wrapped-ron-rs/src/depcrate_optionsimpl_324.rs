// Generated macro for impl_324 (impl)
macro_rules! Depcrate_optionsimpl_324 {
() => {
// Module: crate::options
// Provides: {"impl_324"}
// Dependencies: {}
impl Options { # [must_use] # [doc = " Enable `default_extension` by default during serialization and deserialization."] pub fn with_default_extension (mut self , default_extension : Extensions) -> Self { self . default_extensions |= default_extension ; self } # [must_use] # [doc = " Do NOT enable `default_extension` by default during serialization and deserialization."] pub fn without_default_extension (mut self , default_extension : Extensions) -> Self { self . default_extensions &= ! default_extension ; self } # [must_use] # [doc = " Set a maximum recursion limit during serialization and deserialization."] pub fn with_recursion_limit (mut self , recursion_limit : usize) -> Self { self . recursion_limit = Some (recursion_limit) ; self } # [must_use] # [doc = " Disable the recursion limit during serialization and deserialization."] # [doc = ""] # [doc = " If you expect to handle highly recursive datastructures, consider wrapping"] # [doc = " `ron` with [`serde_stacker`](https://docs.rs/serde_stacker/latest/serde_stacker/)."] pub fn without_recursion_limit (mut self) -> Self { self . recursion_limit = None ; self } }
};
}
