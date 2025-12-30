// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl FromStr for Ownership { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "owning" => Ok (Self :: Owning) , "borrowing" => Ok (Self :: Borrowing { duplicate_if_necessary : false , }) , "borrowing-duplicate-if-necessary" => Ok (Self :: Borrowing { duplicate_if_necessary : true , }) , _ => Err (format ! ("unrecognized ownership: `{s}`; \
                 expected `owning`, `borrowing`, or `borrowing-duplicate-if-necessary`")) , } } }
};
}
