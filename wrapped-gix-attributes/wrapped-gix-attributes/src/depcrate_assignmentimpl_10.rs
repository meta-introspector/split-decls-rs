// Generated macro for impl_10 (impl)
macro_rules! Depcrate_assignmentimpl_10 {
() => {
// Module: crate::assignment
// Provides: {"impl_10"}
// Dependencies: {}
impl std :: fmt :: Display for AssignmentRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self . state { StateRef :: Set => f . write_str (self . name . as_str ()) , StateRef :: Unset => { f . write_char ('-') ? ; f . write_str (self . name . as_str ()) } StateRef :: Value (v) => { f . write_str (self . name . as_str ()) ? ; f . write_char ('=') ? ; f . write_str (v . as_bstr () . to_str_lossy () . as_ref ()) } StateRef :: Unspecified => { f . write_char ('!') ? ; f . write_str (self . name . as_str ()) } } } }
};
}
