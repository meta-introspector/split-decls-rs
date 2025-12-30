// Generated macro for impl_437 (impl)
macro_rules! Depcrate_request_stubimpl_437 {
() => {
// Module: crate::request_stub
// Provides: {"impl_437"}
// Dependencies: {}
impl < T : std :: fmt :: Display > Display for NaOption < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . inner . as_ref () { Some (v) => write ! (f , "{}" , v) , None => write ! (f , "n/a") , } } }
};
}
