// Generated macro for assert_capacity_limit (macro)
macro_rules! Depcrateassert_capacity_limit {
() => {
// Module: crate
// Provides: {"assert_capacity_limit"}
// Dependencies: {}
macro_rules ! assert_capacity_limit { ($ cap : expr) => { if std :: mem :: size_of ::< usize > () > std :: mem :: size_of ::< LenUint > () { if $ cap > LenUint :: MAX as usize { # [cfg (not (target_pointer_width = "16"))] panic ! ("ArrayVec: largest supported capacity is u32::MAX") ; # [cfg (target_pointer_width = "16")] panic ! ("ArrayVec: largest supported capacity is u16::MAX") ; } } } ; }
};
}
