// Generated macro for impl_14 (impl)
macro_rules! Depcrate_bufreaderimpl_14 {
() => {
// Module: crate::bufreader
// Provides: {"impl_14"}
// Dependencies: {}
impl < R : Read > BufReader < R > { pub fn new (inner : R) -> BufReader < R > { BufReader :: with_buf (vec ! [0 ; 32 * 1024] , inner) } pub fn with_buf (buf : Vec < u8 > , inner : R) -> BufReader < R > { BufReader { inner , buf : buf . into_boxed_slice () , pos : 0 , cap : 0 , } } }
};
}
