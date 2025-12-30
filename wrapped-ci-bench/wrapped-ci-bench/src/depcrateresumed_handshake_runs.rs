// Generated macro for RESUMED_HANDSHAKE_RUNS (const)
macro_rules! DepcrateRESUMED_HANDSHAKE_RUNS {
() => {
// Module: crate
// Provides: {"RESUMED_HANDSHAKE_RUNS"}
// Dependencies: {}
# [doc = " The amount of times a resumed handshake should be executed during benchmarking."] # [doc = ""] # [doc = " Handshakes with session resumption execute a very small amount of instructions (less than 200_000"] # [doc = " for some parameters), so a small difference in instructions accounts for a high difference in"] # [doc = " percentage (making the benchmark more sensitive to noise, because differences as low as 500"] # [doc = " instructions already raise a flag). Running the handshake multiple times gives additional weight"] # [doc = " to the instructions involved in the handshake, and less weight to noisy one-time setup code."] # [doc = ""] # [doc = " More specifically, great part of the noise in resumed handshakes comes from the usage of"] # [doc = " [`rustls::client::ClientSessionMemoryCache`] and [`rustls::server::ServerSessionMemoryCache`],"] # [doc = " which rely on a randomized `HashMap` under the hood (you can check for yourself by that"] # [doc = " `HashMap` by a `FxHashMap`, which brings the noise down to acceptable levels in a single run)."] const RESUMED_HANDSHAKE_RUNS : usize = 30 ;
};
}
