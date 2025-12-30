// Generated macro for derive_max_size (function)
macro_rules! Depcratederive_max_size {
() => {
// Module: crate
// Provides: {"derive_max_size"}
// Dependencies: {}
# [doc = " Derive the `postcard::MaxSize` trait for a struct or enum."] # [proc_macro_derive (MaxSize)] pub fn derive_max_size (item : proc_macro :: TokenStream) -> proc_macro :: TokenStream { max_size :: do_derive_max_size (item) }
};
}
