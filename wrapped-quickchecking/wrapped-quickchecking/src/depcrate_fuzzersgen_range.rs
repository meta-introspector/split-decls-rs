// Generated macro for gen_range (function)
macro_rules! Depcrate_fuzzersgen_range {
() => {
// Module: crate::fuzzers
// Provides: {"gen_range"}
// Dependencies: {}
# [doc = " FIXME: is this actually uniform?"] fn gen_range (gen : & mut Gen , lo : u64 , hi : u64) -> u64 { let len = hi - lo ; (u64 :: arbitrary (gen) % len) + lo }
};
}
