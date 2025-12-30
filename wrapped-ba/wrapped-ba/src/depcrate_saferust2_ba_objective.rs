// Generated macro for rust2_ba_objective (function)
macro_rules! Depcrate_saferust2_ba_objective {
() => {
// Module: crate::safe
// Provides: {"rust2_ba_objective"}
// Dependencies: {}
# [no_mangle] extern "C" fn rust2_ba_objective (n : i32 , m : i32 , p : i32 , cams : * const f64 , x : * const f64 , w : * const f64 , obs : * const i32 , feats : * const f64 , reproj_err : * mut f64 , w_err : * mut f64 ,) { let n = n as usize ; let m = m as usize ; let p = p as usize ; let cams = unsafe { std :: slice :: from_raw_parts (cams , n * 11) } ; let x = unsafe { std :: slice :: from_raw_parts (x , m * 3) } ; let w = unsafe { std :: slice :: from_raw_parts (w , p) } ; let obs = unsafe { std :: slice :: from_raw_parts (obs , p * 2) } ; let feats = unsafe { std :: slice :: from_raw_parts (feats , p * 2) } ; let reproj_err = unsafe { std :: slice :: from_raw_parts_mut (reproj_err , p * 2) } ; let w_err = unsafe { std :: slice :: from_raw_parts_mut (w_err , p) } ; rust_ba_objective (n , m , p , cams , x , w , obs , feats , reproj_err , w_err) ; }
};
}
