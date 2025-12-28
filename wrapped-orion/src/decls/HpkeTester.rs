macro_rules! deps {
    () => {
        TestableHpke!();
    };
}

macro_rules! HpkeTester {
    () => {
        deps!();
        pub struct HpkeTester < T : TestableHpke > { hpke_sender : T , hpke_recipient : T , rng : SmallRng , }
    };
}

HpkeTester!()