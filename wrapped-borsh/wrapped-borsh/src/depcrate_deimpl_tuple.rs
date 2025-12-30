// Generated macro for impl_tuple (macro)
macro_rules! Depcrate_deimpl_tuple {
() => {
// Module: crate::de
// Provides: {"impl_tuple"}
// Dependencies: {}
macro_rules ! impl_tuple { (@ unit $ name : ty) => { impl BorshDeserialize for $ name { # [inline] fn deserialize_reader < R : Read > (_reader : & mut R) -> Result < Self > { Ok (<$ name >:: default ()) } } } ; ($ ($ name : ident) +) => { impl <$ ($ name) ,+> BorshDeserialize for ($ ($ name ,) +) where $ ($ name : BorshDeserialize ,) + { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { Ok (($ ($ name :: deserialize_reader (reader) ?,) +)) } } } ; }
};
}
