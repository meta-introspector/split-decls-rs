// Generated macro for impl_115 (impl)
macro_rules! Depcrateimpl_115 {
() => {
// Module: crate
// Provides: {"impl_115"}
// Dependencies: {}
impl Mul < u64 > for Size { type Output = Size ; # [inline] fn mul (self , count : u64) -> Size { match self . bytes () . checked_mul (count) { Some (bytes) => Size :: from_bytes (bytes) , None => panic ! ("Size::mul: {} * {} doesn't fit in u64" , self . bytes () , count) , } } }
};
}
