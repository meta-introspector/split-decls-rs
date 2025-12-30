// Generated macro for rust_ba_objective (function)
macro_rules! Depcrate_saferust_ba_objective {
() => {
// Module: crate::safe
// Provides: {"rust_ba_objective"}
// Dependencies: {}
fn rust_ba_objective (n : usize , m : usize , p : usize , cams : & [f64] , x : & [f64] , w : & [f64] , obs : & [i32] , feats : & [f64] , reproj_err : & mut [f64] , w_err : & mut [f64] ,) { assert_eq ! (cams . len () , n * 11) ; assert_eq ! (x . len () , m * 3) ; assert_eq ! (w . len () , p) ; assert_eq ! (obs . len () , p * 2) ; assert_eq ! (feats . len () , p * 2) ; assert_eq ! (reproj_err . len () , p * 2) ; assert_eq ! (w_err . len () , p) ; for i in 0 .. p { let cam_idx = obs [i * 2 + 0] as usize ; let pt_idx = obs [i * 2 + 1] as usize ; let start = cam_idx * BA_NCAMPARAMS ; let cam : & [f64 ; 11] = unsafe { cams [start ..] . get_unchecked (.. 11) . try_into () . unwrap_unchecked () } ; let x : & [f64 ; 3] = unsafe { x [pt_idx * 3 ..] . get_unchecked (.. 3) . try_into () . unwrap_unchecked () } ; let w : & [f64 ; 1] = unsafe { w [i ..] . get_unchecked (.. 1) . try_into () . unwrap_unchecked () } ; let feat : & [f64 ; 2] = unsafe { feats [i * 2 ..] . get_unchecked (.. 2) . try_into () . unwrap_unchecked () } ; let reproj_err : & mut [f64 ; 2] = unsafe { reproj_err [i * 2 ..] . get_unchecked_mut (.. 2) . try_into () . unwrap_unchecked () } ; compute_reproj_error (cam , x , w , feat , reproj_err) ; } for i in 0 .. p { let w_err : & mut f64 = unsafe { w_err . get_unchecked_mut (i) } ; compute_zach_weight_error (w [i ..] . as_ptr () , w_err as * mut f64) ; } }
};
}
