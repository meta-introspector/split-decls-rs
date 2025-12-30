// Generated macro for radix2 (function)
macro_rules! Depcrate_saferadix2 {
() => {
// Module: crate::safe
// Provides: {"radix2"}
// Dependencies: {}
fn radix2 (data : & mut [f64] , i_sign : i32) { let n = data . len () / 2 ; if n == 1 { return ; } let (a , b) = data . split_at_mut (n) ; radix2 (a , i_sign) ; radix2 (b , i_sign) ; let wtemp = i_sign as f64 * (PI / n as f64) . sin () ; let wpi = - i_sign as f64 * (2.0 * (PI / n as f64)) . sin () ; let wpr = - 2.0 * wtemp * wtemp ; let mut wr = 1.0 ; let mut wi = 0.0 ; let (achunks , _) = a . as_chunks_mut () ; let (bchunks , _) = b . as_chunks_mut () ; for ([ax , ay] , [bx , by]) in achunks . iter_mut () . zip (bchunks . iter_mut ()) { let tempr = * bx * wr - * by * wi ; let tempi = * bx * wi + * by * wr ; * bx = * ax - tempr ; * by = * ay - tempi ; * ax += tempr ; * ay += tempi ; let wtemp_new = wr ; wr = wr * (wpr + 1.0) - wi * wpi ; wi = wi * (wpr + 1.0) + wtemp_new * wpi ; } }
};
}
