// Generated macro for impl_77 (impl)
macro_rules! Depcrate_borrowimpl_77 {
() => {
// Module: crate::borrow
// Provides: {"impl_77"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < B : ? Sized > Ord for Cow < '_ , B > where B : Ord + ToOwned , { # [inline] fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& * * self , & * * other) } }
};
}
