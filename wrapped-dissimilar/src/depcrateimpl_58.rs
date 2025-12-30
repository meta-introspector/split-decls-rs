// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl Debug for Chunk < '_ > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let (name , text) = match * self { Chunk :: Equal (text) => ("Equal" , text) , Chunk :: Delete (text) => ("Delete" , text) , Chunk :: Insert (text) => ("Insert" , text) , } ; write ! (formatter , "{}({:?})" , name , text) } }
};
}
