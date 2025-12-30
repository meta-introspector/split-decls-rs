// Generated macro for impl_109 (impl)
macro_rules! Depcrate_sectionimpl_109 {
() => {
// Module: crate::section
// Provides: {"impl_109"}
// Dependencies: {}
impl < T > SectionExt for T where T : Display + Send + Sync + 'static , { fn header < C > (self , header : C) -> IndentedSection < C , Self > where C : Display + Send + Sync + 'static , { IndentedSection { body : self , header } } }
};
}
