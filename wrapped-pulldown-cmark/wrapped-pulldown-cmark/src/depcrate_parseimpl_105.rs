// Generated macro for impl_105 (impl)
macro_rules! Depcrate_parseimpl_105 {
() => {
// Module: crate::parse
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'a > LinkDef < 'a > { pub fn into_static (self) -> LinkDef < 'static > { LinkDef { dest : self . dest . into_static () , title : self . title . map (| s | s . into_static ()) , span : self . span , } } }
};
}
