macro_rules! deps {
    () => {
        TraitBoundModifiers!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl TraitBoundModifiers { pub const NONE : Self = TraitBoundModifiers { constness : BoundConstness :: Never , polarity : BoundPolarity :: Positive } ; }
    };
}

impl_132!();