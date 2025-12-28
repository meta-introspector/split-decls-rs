macro_rules! deps {
    () => {
        LocalRef!();
    };
}

macro_rules! Locals {
    () => {
        deps!();
        pub (super) struct Locals < 'tcx , V > { values : IndexVec < mir :: Local , LocalRef < 'tcx , V > > , }
    };
}

Locals!()