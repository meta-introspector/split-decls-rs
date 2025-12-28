macro_rules! deps {
    () => {
        StableSince!();
        Stability!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl Stability { pub fn is_unstable (& self) -> bool { self . level . is_unstable () } pub fn is_stable (& self) -> bool { self . level . is_stable () } pub fn stable_since (& self) -> Option < StableSince > { self . level . stable_since () } }
    };
}

impl_445!();