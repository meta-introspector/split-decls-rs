// Generated macro for impl_444 (impl)
macro_rules! Depcrate_objects_jlistimpl_444 {
() => {
// Module: crate::objects::jlist
// Provides: {"impl_444"}
// Dependencies: {}
impl < 'local > From < JList < 'local > > for JCollection < 'local > { fn from (other : JList < 'local >) -> JCollection < 'local > { unsafe { JCollection :: kind_from_raw (other . into_raw ()) } } }
};
}
