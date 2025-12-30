// Generated macro for impl_44 (impl)
macro_rules! Depcrate_errorsimpl_44 {
() => {
// Module: crate::errors
// Provides: {"impl_44"}
// Dependencies: {}
impl std :: fmt :: Display for LocalizationError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Bundle { error } => write ! (f , "[fluent][bundle] error: {}" , error) , Self :: Resolver { id , locale , errors } => { let errors : Vec < String > = errors . iter () . map (| err | err . to_string ()) . collect () ; write ! (f , "[fluent][resolver] errors in {}/{}: {}" , locale , id , errors . join (", ")) } Self :: MissingMessage { id , locale : Some (locale) , } => write ! (f , "[fluent] Missing message in locale {}: {}" , locale , id) , Self :: MissingMessage { id , locale : None } => { write ! (f , "[fluent] Couldn't find a message: {}" , id) } Self :: MissingValue { id , locale : Some (locale) , } => write ! (f , "[fluent] Message has no value in locale {}: {}" , locale , id) , Self :: MissingValue { id , locale : None } => { write ! (f , "[fluent] Couldn't find a message with value: {}" , id) } Self :: SyncRequestInAsyncMode => { write ! (f , "Triggered synchronous format while in async mode") } } } }
};
}
