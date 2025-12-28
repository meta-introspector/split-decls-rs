macro_rules! deps {
    () => {
        InternalNumeric!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl defmt :: Format for InternalNumeric { fn format (& self , f : defmt :: Formatter) { defmt :: write ! (f , "<InternalNumeric>") } }
    };
}

impl_262!();