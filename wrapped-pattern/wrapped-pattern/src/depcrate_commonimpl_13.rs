// Generated macro for impl_13 (impl)
macro_rules! Depcrate_commonimpl_13 {
() => {
// Module: crate::common
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , T , U > From < PatternItem < 'a , U > > for PatternItemCow < 'a , T > where T : From < U > , { fn from (value : PatternItem < 'a , U >) -> Self { match value { PatternItem :: Placeholder (t) => Self :: Placeholder (t . into ()) , PatternItem :: Literal (s) => Self :: Literal (Cow :: Borrowed (s)) , } } }
};
}
