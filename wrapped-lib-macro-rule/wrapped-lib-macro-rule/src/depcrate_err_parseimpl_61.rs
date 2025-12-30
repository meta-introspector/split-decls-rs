// Generated macro for impl_61 (impl)
macro_rules! Depcrate_err_parseimpl_61 {
() => {
// Module: crate::err_parse
// Provides: {"impl_61"}
// Dependencies: {}
impl < T > ParseResultExt < T > for ErrParse < T > { fn map < U , F > (self , _op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static { Box :: new (ErrParse :: new (self . failures)) } }
};
}
