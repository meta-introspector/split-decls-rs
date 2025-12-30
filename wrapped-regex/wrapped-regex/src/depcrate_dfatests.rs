// Generated macro for tests (module)
macro_rules! Depcrate_dfatests {
() => {
// Module: crate::dfa
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { extern crate rand ; use quickcheck :: { QuickCheck , StdGen , quickcheck } ; use super :: { StateFlags , State , push_inst_ptr , write_varu32 , read_varu32 , write_vari32 , read_vari32 , } ; # [test] fn prop_state_encode_decode () { fn p (ips : Vec < u32 > , flags : u8) -> bool { let mut data = vec ! [flags] ; let mut prev = 0 ; for & ip in ips . iter () { push_inst_ptr (& mut data , & mut prev , ip) ; } let state = State { data : data . into_boxed_slice () } ; let expected : Vec < usize > = ips . into_iter () . map (| ip | ip as usize) . collect () ; let got : Vec < usize > = state . inst_ptrs () . collect () ; expected == got && state . flags () == StateFlags (flags) } QuickCheck :: new () . gen (StdGen :: new (self :: rand :: thread_rng () , 10_000)) . quickcheck (p as fn (Vec < u32 > , u8) -> bool) ; } # [test] fn prop_read_write_u32 () { fn p (n : u32) -> bool { let mut buf = vec ! [] ; write_varu32 (& mut buf , n) ; let (got , nread) = read_varu32 (& buf) ; nread == buf . len () && got == n } quickcheck (p as fn (u32) -> bool) ; } # [test] fn prop_read_write_i32 () { fn p (n : i32) -> bool { let mut buf = vec ! [] ; write_vari32 (& mut buf , n) ; let (got , nread) = read_vari32 (& buf) ; nread == buf . len () && got == n } quickcheck (p as fn (i32) -> bool) ; } }
};
}
