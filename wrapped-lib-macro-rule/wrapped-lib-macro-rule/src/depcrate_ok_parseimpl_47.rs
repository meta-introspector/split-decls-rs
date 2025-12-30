// Generated macro for impl_47 (impl)
macro_rules! Depcrate_ok_parseimpl_47 {
() => {
// Module: crate::ok_parse
// Provides: {"impl_47"}
// Dependencies: {}
impl < T > ParseResultExt < T > for OkParse < T > { fn map < U , F > (self , op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static { Box :: new (OkParse (op (self . 0))) } }
};
}
