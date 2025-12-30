// Generated macro for macro_22 (macro)
macro_rules! Depcrate_float_cmpmacro_22 {
() => {
// Module: crate::float::cmp
// Provides: {"macro_22"}
// Dependencies: {}
# [cfg (target_arch = "arm")] intrinsics ! { pub extern "aapcs" fn __aeabi_fcmple (a : f32 , b : f32) -> i32 { (__lesf2 (a , b) <= 0) as i32 } pub extern "aapcs" fn __aeabi_fcmpge (a : f32 , b : f32) -> i32 { (__gesf2 (a , b) >= 0) as i32 } pub extern "aapcs" fn __aeabi_fcmpeq (a : f32 , b : f32) -> i32 { (__eqsf2 (a , b) == 0) as i32 } pub extern "aapcs" fn __aeabi_fcmplt (a : f32 , b : f32) -> i32 { (__ltsf2 (a , b) < 0) as i32 } pub extern "aapcs" fn __aeabi_fcmpgt (a : f32 , b : f32) -> i32 { (__gtsf2 (a , b) > 0) as i32 } pub extern "aapcs" fn __aeabi_dcmple (a : f64 , b : f64) -> i32 { (__ledf2 (a , b) <= 0) as i32 } pub extern "aapcs" fn __aeabi_dcmpge (a : f64 , b : f64) -> i32 { (__gedf2 (a , b) >= 0) as i32 } pub extern "aapcs" fn __aeabi_dcmpeq (a : f64 , b : f64) -> i32 { (__eqdf2 (a , b) == 0) as i32 } pub extern "aapcs" fn __aeabi_dcmplt (a : f64 , b : f64) -> i32 { (__ltdf2 (a , b) < 0) as i32 } pub extern "aapcs" fn __aeabi_dcmpgt (a : f64 , b : f64) -> i32 { (__gtdf2 (a , b) > 0) as i32 } }
};
}
