macro_rules! deps {
    () => {
        OneTraitImpls!();
    };
}

macro_rules! TraitImpls {
    () => {
        deps!();
        # [derive (Debug , PartialEq)] pub struct TraitImpls { map : FxHashMap < TraitId , OneTraitImpls > , }
    };
}

TraitImpls!()