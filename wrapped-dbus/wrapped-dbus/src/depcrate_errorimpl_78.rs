// Generated macro for impl_78 (impl)
macro_rules! Depcrate_errorimpl_78 {
() => {
// Module: crate::error
// Provides: {"impl_78"}
// Dependencies: {}
impl < T : Into < ErrorName < 'static > > , M : Into < String > > From < (T , M) > for MethodErr { fn from ((t , m) : (T , M)) -> MethodErr { MethodErr (t . into () , m . into ()) } }
};
}
