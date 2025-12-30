// Generated macro for impl_110 (impl)
macro_rules! Depcrateimpl_110 {
() => {
// Module: crate
// Provides: {"impl_110"}
// Dependencies: {}
impl Sub for Size { type Output = Size ; # [inline] fn sub (self , other : Size) -> Size { Size :: from_bytes (self . bytes () . checked_sub (other . bytes ()) . unwrap_or_else (| | { panic ! ("Size::sub: {} - {} would result in negative size" , self . bytes () , other . bytes ()) })) } }
};
}
