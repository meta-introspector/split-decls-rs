// Generated macro for test (module)
macro_rules! Depcrate_samplingtest {
() => {
// Module: crate::sampling
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: as_conversions)] # [allow (clippy :: cast_possible_truncation)] mod test { use super :: * ; use hybrid_array :: typenum :: { U16 , U256 } ; fn max_abs_1 (p : & Polynomial) -> bool { p . 0 . iter () . all (| x | x . 0 == 0 || x . 0 == 1 || x . 0 == BaseField :: Q - 1) } fn hamming_weight (p : & Polynomial) -> usize { p . 0 . iter () . filter (| x | x . 0 != 0) . count () } # [test] fn test_sample_in_ball () { for tau in 1 .. 65 { for seed in 0_usize .. 255 { let rho = ((tau as u16) << 8) + (seed as u16) ; let p = sample_in_ball (& rho . to_be_bytes () , tau) ; assert_eq ! (hamming_weight (& p) , tau) ; assert ! (max_abs_1 (& p)) ; } } } # [test] fn test_rej_ntt_poly () { let sample : Array < Array < Elem , U256 > , U16 > = Array :: from_fn (| i | { let i = i as u8 ; let rho = [i ; 32] ; rej_ntt_poly (& rho , i , i + 1) . 0 }) ; let sample = sample . as_flattened () ; let all_in_range = sample . iter () . all (| x | x . 0 < BaseField :: Q) ; assert ! (all_in_range) ; } # [test] fn test_sample_cbd () { let rho = [0 ; 32] ; let sample = rej_bounded_poly (& rho , Eta :: Two , 0) . 0 ; let all_in_range = sample . iter () . map (| x | * x + Elem :: new (2)) . all (| x | x . 0 < 5) ; assert ! (all_in_range) ; let sample = rej_bounded_poly (& rho , Eta :: Four , 0) . 0 ; let all_in_range = sample . iter () . map (| x | * x + Elem :: new (4)) . all (| x | x . 0 < 9) ; assert ! (all_in_range) ; } }
};
}
