// Generated macro for impl_149 (impl)
macro_rules! Depcrateimpl_149 {
() => {
// Module: crate
// Provides: {"impl_149"}
// Dependencies: {}
impl PosixDayTime { fn quote (& self) -> proc_macro2 :: TokenStream { let PosixDayTime { ref date , ref time } = * self ; let date = date . quote () ; let time = time . quote () ; quote ! { jiff :: shared :: PosixDayTime { date : # date , time : # time } } } }
};
}
