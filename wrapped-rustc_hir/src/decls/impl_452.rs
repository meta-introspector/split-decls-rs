macro_rules! deps {
    () => {
        StabilityLevel!();
        StableSince!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        impl StabilityLevel { pub fn is_unstable (& self) -> bool { matches ! (self , StabilityLevel :: Unstable { .. }) } pub fn is_stable (& self) -> bool { matches ! (self , StabilityLevel :: Stable { .. }) } pub fn stable_since (& self) -> Option < StableSince > { match * self { StabilityLevel :: Stable { since , .. } => Some (since) , StabilityLevel :: Unstable { .. } => None , } } }
    };
}

impl_452!();