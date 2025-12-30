// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl fmt :: Display for Ownership { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match self { Ownership :: Owning => "owning" , Ownership :: CoarseBorrowing => "coarse-borrowing" , Ownership :: FineBorrowing => "fine-borrowing" , }) } }
};
}
