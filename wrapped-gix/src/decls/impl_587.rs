macro_rules! deps {
    () => {
        Key!();
        Branch!();
        Section!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl Section for Branch { fn name (& self) -> & str { "branch" } fn keys (& self) -> & [& dyn Key] { & [& Self :: MERGE , & Self :: PUSH_REMOTE , & Self :: REMOTE] } }
    };
}

impl_587!();