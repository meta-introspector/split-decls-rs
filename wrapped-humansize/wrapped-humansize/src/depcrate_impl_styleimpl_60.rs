// Generated macro for impl_60 (impl)
macro_rules! Depcrate_impl_styleimpl_60 {
() => {
// Module: crate::impl_style
// Provides: {"impl_60"}
// Dependencies: {}
impl < T : ToF64 + Signed + Copy > FormatSizeI < T > for T { fn format_size_i (& self , opts : FormatSizeOptions) -> String { format ! ("{}" , ISizeFormatter :: new (* self , opts)) } }
};
}
