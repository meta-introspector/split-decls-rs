// Generated macro for impl_749 (impl)
macro_rules! Depcrate_characterimpl_749 {
() => {
// Module: crate::character
// Provides: {"impl_749"}
// Dependencies: {}
impl < I : Input , E : ParseError < I > > Parser < I > for Digit1 < E > where < I as Input > :: Item : AsChar , { type Output = I ; type Error = E ; # [inline] fn process < OM : crate :: OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { input . split_at_position_mode1 :: < OM , _ , _ > (| item | ! item . is_dec_digit () , ErrorKind :: Digit) } }
};
}
