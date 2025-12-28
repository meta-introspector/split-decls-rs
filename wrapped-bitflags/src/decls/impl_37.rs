macro_rules! deps {
    () => {
        Flags!();
        IterNames!();
        BitFlags!();
        Iter!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        # [allow (deprecated)] impl < B : Flags > BitFlags for B { type Iter = iter :: Iter < Self > ; type IterNames = iter :: IterNames < Self > ; }
    };
}

impl_37!()