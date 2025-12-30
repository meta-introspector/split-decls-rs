// Generated macro for impl_59 (impl)
macro_rules! Depcrate_impl_styleimpl_59 {
() => {
// Module: crate::impl_style
// Provides: {"impl_59"}
// Dependencies: {}
impl < T : ToF64 + Unsigned + Copy > FormatSize < T > for T { fn format_size (& self , opts : FormatSizeOptions) -> String { format ! ("{}" , SizeFormatter :: new (* self , opts)) } }
};
}
