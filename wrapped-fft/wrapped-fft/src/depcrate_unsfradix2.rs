// Generated macro for radix2 (function)
macro_rules! Depcrate_unsfradix2 {
() => {
// Module: crate::unsf
// Provides: {"radix2"}
// Dependencies: {}
unsafe fn radix2 (data : * mut f64 , n : usize , i_sign : i32) { if n == 1 { return ; } radix2 (data , n / 2 , i_sign) ; radix2 (data . add (n) , n / 2 , i_sign) ; let wtemp = i_sign as f64 * (PI / n as f64) . sin () ; let wpi = - i_sign as f64 * (2.0 * (PI / n as f64)) . sin () ; let wpr = - 2.0 * wtemp * wtemp ; let mut wr = 1.0 ; let mut wi = 0.0 ; for i in (0 .. n) . step_by (2) { let in_n = i + n ; let ax = & mut * data . add (i) ; let ay = & mut * data . add (i + 1) ; let bx = & mut * data . add (in_n) ; let by = & mut * data . add (in_n + 1) ; let tempr = * bx * wr - * by * wi ; let tempi = * bx * wi + * by * wr ; * bx = * ax - tempr ; * by = * ay - tempi ; * ax += tempr ; * ay += tempi ; let wtemp_new = wr ; wr = wr * (wpr + 1.0) - wi * wpi ; wi = wi * (wpr + 1.0) + wtemp_new * wpi ; } }
};
}
