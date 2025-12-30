// Generated macro for impl_74 (impl)
macro_rules! Depcrate_errorimpl_74 {
() => {
// Module: crate::error
// Provides: {"impl_74"}
// Dependencies: {}
impl CargoError { pub (crate) fn new (kind : ErrorKind) -> Self { Self { kind , context : None , cause : None , } } pub (crate) fn set_context < S > (mut self , context : S) -> Self where S : Into < String > , { let context = context . into () ; self . context = Some (context) ; self } pub (crate) fn set_cause < E > (mut self , cause : E) -> Self where E : Error + Send + Sync + 'static , { let cause = Box :: new (cause) ; self . cause = Some (cause) ; self } # [doc = " For programmatically processing failures."] pub fn kind (& self) -> ErrorKind { self . kind } }
};
}
