// Generated macro for impl_208 (impl)
macro_rules! Depcrate_graph_scc_testsimpl_208 {
() => {
// Module: crate::graph::scc::tests
// Provides: {"impl_208"}
// Dependencies: {}
impl Annotations < usize > for Maxes { fn new (& self , element : usize) -> MaxReached { MaxReached (self . 1 (element)) } fn annotate_scc (& mut self , scc : usize , annotation : MaxReached) { let i = self . 0 . push (annotation) ; assert ! (i == scc) ; } type Ann = MaxReached ; type SccIdx = usize ; }
};
}
