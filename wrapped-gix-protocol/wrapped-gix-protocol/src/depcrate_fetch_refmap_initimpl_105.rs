// Generated macro for impl_105 (impl)
macro_rules! Depcrate_fetch_refmap_initimpl_105 {
() => {
// Module: crate::fetch::refmap::init
// Provides: {"impl_105"}
// Dependencies: {}
impl Context { fn aggregate_refspecs (& self) -> Vec < gix_refspec :: RefSpec > { let mut all_refspecs = self . fetch_refspecs . clone () ; all_refspecs . extend (self . extra_refspecs . iter () . cloned ()) ; all_refspecs } }
};
}
