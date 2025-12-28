macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Spec { # [doc = " Return the kind of this specification."] pub fn kind (& self) -> Kind { match self { Spec :: Include (_) => Kind :: IncludeReachable , Spec :: Exclude (_) => Kind :: ExcludeReachable , Spec :: Range { .. } => Kind :: RangeBetween , Spec :: Merge { .. } => Kind :: ReachableToMergeBase , Spec :: IncludeOnlyParents { .. } => Kind :: IncludeReachableFromParents , Spec :: ExcludeParents { .. } => Kind :: ExcludeReachableFromParents , } } }
    };
}

impl_28!()