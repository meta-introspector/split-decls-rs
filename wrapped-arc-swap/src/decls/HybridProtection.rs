macro_rules! deps {
    () => {
        RefCnt!();
        Debt!();
    };
}

macro_rules! HybridProtection {
    () => {
        deps!();
        pub struct HybridProtection < T : RefCnt > { debt : Option < & 'static Debt > , ptr : ManuallyDrop < T > , }
    };
}

HybridProtection!();