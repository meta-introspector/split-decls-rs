// Generated macro for impl_30 (impl)
macro_rules! Depcrate_fragmentsimpl_30 {
() => {
// Module: crate::fragments
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a > BinaryCollector < 'a > { fn try_catch (& mut self , f : impl FnOnce (& mut BinaryBuf < 'a >) -> Result < () , Error > ,) -> sval :: Result { match f (& mut self . buf) { Ok (()) => Ok (()) , Err (e) => self . fail (e) , } } fn fail (& mut self , err : Error) -> sval :: Result { self . err = Some (err) ; sval :: error () } }
};
}
