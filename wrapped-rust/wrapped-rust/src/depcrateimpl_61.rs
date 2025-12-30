// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl fmt :: Display for Ownership { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match self { Ownership :: Owning => "owning" , Ownership :: Borrowing { duplicate_if_necessary : false , } => "borrowing" , Ownership :: Borrowing { duplicate_if_necessary : true , } => "borrowing-duplicate-if-necessary" , }) } }
};
}
