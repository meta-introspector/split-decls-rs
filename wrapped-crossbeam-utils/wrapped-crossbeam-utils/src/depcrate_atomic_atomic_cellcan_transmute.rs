// Generated macro for can_transmute (function)
macro_rules! Depcrate_atomic_atomic_cellcan_transmute {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"can_transmute"}
// Dependencies: {}
# [doc = " Returns `true` if values of type `A` can be transmuted into values of type `B`."] const fn can_transmute < A , B > () -> bool { (mem :: size_of :: < A > () == mem :: size_of :: < B > ()) & (mem :: align_of :: < A > () >= mem :: align_of :: < B > ()) }
};
}
