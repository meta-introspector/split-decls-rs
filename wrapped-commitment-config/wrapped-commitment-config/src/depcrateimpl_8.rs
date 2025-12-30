// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl fmt :: Display for CommitmentLevel { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let s = match self { CommitmentLevel :: Processed => "processed" , CommitmentLevel :: Confirmed => "confirmed" , CommitmentLevel :: Finalized => "finalized" , } ; write ! (f , "{s}") } }
};
}
