// Generated macro for impl_631 (impl)
macro_rules! Depcrateimpl_631 {
() => {
// Module: crate
// Provides: {"impl_631"}
// Dependencies: {}
# [cfg (not (feature = "equivalent"))] impl < Q : ? Sized , K : ? Sized > Equivalent < K > for Q where Q : Eq , K : core :: borrow :: Borrow < Q > , { fn equivalent (& self , key : & K) -> bool { self == key . borrow () } }
};
}
