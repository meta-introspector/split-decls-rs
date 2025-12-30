// Generated macro for impl_518 (impl)
macro_rules! Depcrate_handle_placeholdersimpl_518 {
() => {
// Module: crate::handle_placeholders
// Provides: {"impl_518"}
// Dependencies: {}
impl scc :: Annotations < RegionVid > for SccAnnotations < '_ , '_ , RegionTracker > { fn new (& self , element : RegionVid) -> RegionTracker { RegionTracker :: new (element , & self . definitions [element]) } fn annotate_scc (& mut self , scc : ConstraintSccIndex , annotation : RegionTracker) { let idx = self . scc_to_annotation . push (annotation) ; assert ! (idx == scc) ; } type Ann = RegionTracker ; type SccIdx = ConstraintSccIndex ; }
};
}
