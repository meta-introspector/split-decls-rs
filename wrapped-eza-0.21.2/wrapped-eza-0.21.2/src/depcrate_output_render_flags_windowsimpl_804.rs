// Generated macro for impl_804 (impl)
macro_rules! Depcrate_output_render_flags_windowsimpl_804 {
() => {
// Module: crate::output::render::flags_windows
// Provides: {"impl_804"}
// Dependencies: {}
impl f :: Flags { pub fn render (self , style : Style , format : FlagsFormat) -> TextCell { TextCell :: paint (style , if format == FlagsFormat :: Short { flags_to_windows_string (self . 0) } else { flags_to_bsd_string (self . 0) } ,) } }
};
}
