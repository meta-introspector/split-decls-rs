// Generated macro for impl_92 (impl)
macro_rules! Depcrateimpl_92 {
() => {
// Module: crate
// Provides: {"impl_92"}
// Dependencies: {}
impl < I > From < :: nom :: error :: Error < I > > for Error < I > { fn from (e : :: nom :: error :: Error < I >) -> Self { Self { input : e . input , error : e . code . into () , } } }
};
}
