// Generated macro for impl_17 (impl)
macro_rules! Depcrate_utilsimpl_17 {
() => {
// Module: crate::utils
// Provides: {"impl_17"}
// Dependencies: {}
impl < N : Into < String > , S : Into < Signature < 'static > > > From < (N , S) > for Argument { fn from ((n , s) : (N , S)) -> Argument { Argument (Some (n . into ()) , s . into ()) } }
};
}
