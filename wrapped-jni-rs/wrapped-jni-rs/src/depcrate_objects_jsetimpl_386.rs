// Generated macro for impl_386 (impl)
macro_rules! Depcrate_objects_jsetimpl_386 {
() => {
// Module: crate::objects::jset
// Provides: {"impl_386"}
// Dependencies: {}
impl < 'local > From < JSet < 'local > > for JCollection < 'local > { fn from (other : JSet < 'local >) -> JCollection < 'local > { unsafe { JCollection :: kind_from_raw (other . into_raw ()) } } }
};
}
