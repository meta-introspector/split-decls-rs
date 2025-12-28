macro_rules! deps {
    () => {
        Statistics!();
        SafetyCheck!();
    };
}

macro_rules! Reducer {
    () => {
        deps!();
        pub struct Reducer < 'a , P , E > { progress : OwnShared < Mutable < P > > , check : traverse :: SafetyCheck , then : Instant , entries_seen : usize , stats : traverse :: Statistics , should_interrupt : & 'a AtomicBool , _error : std :: marker :: PhantomData < E > , }
    };
}

Reducer!()