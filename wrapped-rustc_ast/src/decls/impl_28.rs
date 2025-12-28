macro_rules! deps {
    () => {
        TraitBoundModifiers!();
        BoundPolarity!();
        BoundConstness!();
        BoundAsyncness!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl TraitBoundModifiers { pub const NONE : Self = Self { constness : BoundConstness :: Never , asyncness : BoundAsyncness :: Normal , polarity : BoundPolarity :: Positive , } ; }
    };
}

impl_28!()