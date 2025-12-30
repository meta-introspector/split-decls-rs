// Generated macro for impl_438 (impl)
macro_rules! Depcrate_plotimpl_438 {
() => {
// Module: crate::plot
// Provides: {"impl_438"}
// Dependencies: {}
impl < 'a > PlotContext < 'a > { pub fn size (mut self , s : Option < criterion_plot :: Size >) -> PlotContext < 'a > { if let Some (s) = s { self . size = Some ((s . 0 , s . 1)) ; } self } pub fn thumbnail (mut self , value : bool) -> PlotContext < 'a > { self . is_thumbnail = value ; self } pub fn line_comparison_path (& self) -> PathBuf { let mut path = self . context . output_directory . clone () ; path . push (self . id . as_directory_name ()) ; path . push ("report") ; path . push ("lines.svg") ; path } pub fn violin_path (& self) -> PathBuf { let mut path = self . context . output_directory . clone () ; path . push (self . id . as_directory_name ()) ; path . push ("report") ; path . push ("violin.svg") ; path } }
};
}
