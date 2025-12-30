// Generated macro for impl_19 (impl)
macro_rules! Depcrate_fragmentsimpl_19 {
() => {
// Module: crate::fragments
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > TextCollector < 'a > { fn try_catch (& mut self , f : impl FnOnce (& mut TextBuf < 'a >) -> Result < () , Error >) -> sval :: Result { match f (& mut self . buf) { Ok (()) => Ok (()) , Err (e) => self . fail (e) , } } fn fail (& mut self , err : Error) -> sval :: Result { self . err = Some (err) ; sval :: error () } }
};
}
