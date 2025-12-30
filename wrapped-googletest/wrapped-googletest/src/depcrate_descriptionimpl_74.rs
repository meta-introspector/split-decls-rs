// Generated macro for impl_74 (impl)
macro_rules! Depcrate_descriptionimpl_74 {
() => {
// Module: crate::description
// Provides: {"impl_74"}
// Dependencies: {}
impl < T : Into < Cow < 'static , str > > > From < T > for Description { fn from (value : T) -> Self { let mut elements = List :: default () ; elements . push_literal (value . into ()) ; Self { elements , .. Default :: default () } } }
};
}
