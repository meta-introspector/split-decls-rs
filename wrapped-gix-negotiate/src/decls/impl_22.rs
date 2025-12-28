macro_rules! deps {
    () => {
        Negotiator!();
        Noop!();
        Algorithm!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Algorithm { # [doc = " Create an instance of a negotiator which implements this algorithm."] pub fn into_negotiator (self) -> Box < dyn Negotiator > { match & self { Algorithm :: Noop => Box :: new (noop :: Noop) as Box < dyn Negotiator > , Algorithm :: Consecutive => Box :: < consecutive :: Algorithm > :: default () , Algorithm :: Skipping => Box :: < skipping :: Algorithm > :: default () , } } }
    };
}

impl_22!()