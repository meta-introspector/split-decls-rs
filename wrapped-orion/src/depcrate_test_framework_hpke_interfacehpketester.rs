// Generated macro for HpkeTester (struct)
macro_rules! Depcrate_test_framework_hpke_interfaceHpkeTester {
() => {
// Module: crate::test_framework::hpke_interface
// Provides: {"HpkeTester"}
// Dependencies: {}
pub struct HpkeTester < T : TestableHpke > { hpke_sender : T , hpke_recipient : T , rng : SmallRng , }
};
}
