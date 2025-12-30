// Generated macro for impl_111 (impl)
macro_rules! Depcrate_walkimpl_111 {
() => {
// Module: crate::walk
// Provides: {"impl_111"}
// Dependencies: {}
impl < 's , F : FnMut () -> FnVisitor < 's > > ParallelVisitorBuilder < 's > for FnBuilder < F > { fn build (& mut self) -> Box < dyn ParallelVisitor + 's > { let visitor = (self . builder) () ; Box :: new (FnVisitorImp { visitor }) } }
};
}
