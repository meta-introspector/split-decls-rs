// Generated macro for assert_capacity_limit_const (macro)
macro_rules! Depcrateassert_capacity_limit_const {
() => {
// Module: crate
// Provides: {"assert_capacity_limit_const"}
// Dependencies: {}
macro_rules ! assert_capacity_limit_const { ($ cap : expr) => { if std :: mem :: size_of ::< usize > () > std :: mem :: size_of ::< LenUint > () { if $ cap > LenUint :: MAX as usize { [] [$ cap] } } } }
};
}
