// Generated macro for Bencher (struct)
macro_rules! DepcrateBencher {
() => {
// Module: crate
// Provides: {"Bencher"}
// Dependencies: {}
# [doc = " Stand-in for `bencher::Bencher` which uses Criterion.rs to perform the benchmark instead."] pub struct Bencher < 'a , 'b > { pub bytes : u64 , pub bencher : & 'a mut :: criterion :: Bencher < 'b , WallTime > , }
};
}
