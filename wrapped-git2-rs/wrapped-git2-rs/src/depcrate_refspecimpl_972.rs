// Generated macro for impl_972 (impl)
macro_rules! Depcrate_refspecimpl_972 {
() => {
// Module: crate::refspec
// Provides: {"impl_972"}
// Dependencies: {}
impl < 'remote > Binding for Refspec < 'remote > { type Raw = * const raw :: git_refspec ; unsafe fn from_raw (raw : * const raw :: git_refspec) -> Refspec < 'remote > { Refspec { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * const raw :: git_refspec { self . raw } }
};
}
