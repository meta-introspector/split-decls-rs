macro_rules! deps {
    () => {
        TraitOrTraitImpl!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl TraitOrTraitImpl { fn constness (& self) -> Option < Span > { match self { Self :: Trait { constness : Const :: Yes (span) , .. } | Self :: TraitImpl { constness : Const :: Yes (span) , .. } => Some (* span) , _ => None , } } }
    };
}

impl_2!();