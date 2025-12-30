// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl FromStr for Ownership { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "owning" => Ok (Self :: Owning) , "coarse-borrowing" => Ok (Self :: CoarseBorrowing) , "fine-borrowing" => Ok (Self :: FineBorrowing) , _ => Err (format ! ("unrecognized ownership: `{s}`; \
                 expected `owning`, `coarse-borrowing`, or `fine-borrowing`")) , } } }
};
}
