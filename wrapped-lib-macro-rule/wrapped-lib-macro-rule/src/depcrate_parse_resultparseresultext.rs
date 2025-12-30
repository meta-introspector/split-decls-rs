// Generated macro for ParseResultExt (trait)
macro_rules! Depcrate_parse_resultParseResultExt {
() => {
// Module: crate::parse_result
// Provides: {"ParseResultExt"}
// Dependencies: {}
pub trait ParseResultExt < T > : Sized { fn map < U , F > (self , op : F) -> Box < dyn ParseResultBase < U > > where F : FnOnce (T) -> U , U : 'static ; }
};
}
