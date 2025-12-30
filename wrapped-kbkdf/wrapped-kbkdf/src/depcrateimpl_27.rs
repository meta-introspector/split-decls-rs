// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < Prf , K , R > Kbkdf < Prf , K , R > for Feedback < '_ , Prf , K , R > where Prf : Mac + KeyInit , K : KeySizeUser , K :: KeySize : ArraySize + Mul < U8 > , < K :: KeySize as Mul < U8 > > :: Output : Unsigned , Prf :: OutputSize : ArraySize + Mul < U8 > , < Prf :: OutputSize as Mul < U8 > > :: Output : Unsigned , R : sealed :: R , { fn input_iv (& self , ki : & mut Option < Array < u8 , Prf :: OutputSize > >) { if let Some (iv) = self . iv { * ki = Some (iv . clone ()) } } const FEEDBACK_KI : bool = true ; }
};
}
