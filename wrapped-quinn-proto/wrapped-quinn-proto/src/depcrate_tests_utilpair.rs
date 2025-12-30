// Generated macro for Pair (struct)
macro_rules! Depcrate_tests_utilPair {
() => {
// Module: crate::tests::util
// Provides: {"Pair"}
// Dependencies: {}
pub (super) struct Pair { pub (super) server : TestEndpoint , pub (super) client : TestEndpoint , # [doc = " Start time"] epoch : Instant , # [doc = " Current time"] pub (super) time : Instant , # [doc = " Simulates the maximum size allowed for UDP payloads by the link (packets exceeding this size will be dropped)"] pub (super) mtu : usize , # [doc = " Simulates explicit congestion notification"] pub (super) congestion_experienced : bool , pub (super) latency : Duration , # [doc = " Number of spin bit flips"] pub (super) spins : u64 , last_spin : bool , }
};
}
