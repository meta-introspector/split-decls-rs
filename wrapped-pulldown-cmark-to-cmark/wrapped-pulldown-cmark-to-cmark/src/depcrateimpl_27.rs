// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > From < & 'a TableAlignment > for Alignment { fn from (s : & 'a TableAlignment) -> Self { match * s { TableAlignment :: None => Self :: None , TableAlignment :: Left => Self :: Left , TableAlignment :: Center => Self :: Center , TableAlignment :: Right => Self :: Right , } } }
};
}
