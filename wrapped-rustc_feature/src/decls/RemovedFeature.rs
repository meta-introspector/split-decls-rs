macro_rules! deps {
    () => {
        Feature!();
    };
}

macro_rules! RemovedFeature {
    () => {
        deps!();
        pub struct RemovedFeature { pub feature : Feature , pub reason : Option < & 'static str > , pub pull : Option < NonZero < u32 > > , }
    };
}

RemovedFeature!()