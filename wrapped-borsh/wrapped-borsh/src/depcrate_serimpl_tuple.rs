// Generated macro for impl_tuple (macro)
macro_rules! Depcrate_serimpl_tuple {
() => {
// Module: crate::ser
// Provides: {"impl_tuple"}
// Dependencies: {}
macro_rules ! impl_tuple { (@ unit $ name : ty) => { impl BorshSerialize for $ name { # [inline] fn serialize < W : Write > (& self , _writer : & mut W) -> Result < () > { Ok (()) } } } ; ($ ($ idx : tt $ name : ident) +) => { impl <$ ($ name) ,+> BorshSerialize for ($ ($ name ,) +) where $ ($ name : BorshSerialize ,) + { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { $ (self .$ idx . serialize (writer) ?;) + Ok (()) } } } ; }
};
}
