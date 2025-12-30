// Generated macro for fibonacci (function)
macro_rules! Depcratefibonacci {
() => {
// Module: crate
// Provides: {"fibonacci"}
// Dependencies: {}
# [wasm_bindgen] pub fn fibonacci (n : i32) -> i32 { let mut a = 1u64 ; let mut b = 1 ; for _ in 0 .. n { let tmp = b ; b += a ; a = tmp ; } unsafe { FIB_HIGH = (a >> 32) as i32 ; } a as i32 }
};
}
