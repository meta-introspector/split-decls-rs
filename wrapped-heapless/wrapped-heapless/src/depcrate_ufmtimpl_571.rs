// Generated macro for impl_571 (impl)
macro_rules! Depcrate_ufmtimpl_571 {
() => {
// Module: crate::ufmt
// Provides: {"impl_571"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > uDisplay for StringInner < LenT , S > { # [inline] fn fmt < W > (& self , f : & mut ufmt :: Formatter < '_ , W >) -> Result < () , W :: Error > where W : uWrite + ? Sized , { f . write_str (self . as_str ()) } }
};
}
