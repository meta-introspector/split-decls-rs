macro_rules! deps {
    () => {
        WalkEvent!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < T > WalkEvent < T > { pub fn map < F : FnOnce (T) -> U , U > (self , f : F) -> WalkEvent < U > { match self { WalkEvent :: Enter (it) => WalkEvent :: Enter (f (it)) , WalkEvent :: Leave (it) => WalkEvent :: Leave (f (it)) , } } }
    };
}

impl_106!()