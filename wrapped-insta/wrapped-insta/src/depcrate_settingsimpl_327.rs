// Generated macro for impl_327 (impl)
macro_rules! Depcrate_settingsimpl_327 {
() => {
// Module: crate::settings
// Provides: {"impl_327"}
// Dependencies: {}
# [cfg (feature = "redactions")] impl < 'a > From < Vec < (& 'a str , Redaction) > > for Redactions { fn from (value : Vec < (& 'a str , Redaction) >) -> Redactions { Redactions (value . into_iter () . map (| x | (Selector :: parse (x . 0) . unwrap () . make_static () , Arc :: new (x . 1))) . collect () ,) } }
};
}
