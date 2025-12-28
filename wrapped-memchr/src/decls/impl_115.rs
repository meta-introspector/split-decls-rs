macro_rules! deps {
    () => {
        SuffixKind!();
        SuffixOrdering!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl SuffixKind { # [doc = " Returns true if and only if the given candidate byte indicates that"] # [doc = " it should replace the current suffix as the maximal (or minimal)"] # [doc = " suffix."] fn cmp (self , current : u8 , candidate : u8) -> SuffixOrdering { use self :: SuffixOrdering :: * ; match self { SuffixKind :: Minimal if candidate < current => Accept , SuffixKind :: Minimal if candidate > current => Skip , SuffixKind :: Minimal => Push , SuffixKind :: Maximal if candidate > current => Accept , SuffixKind :: Maximal if candidate < current => Skip , SuffixKind :: Maximal => Push , } } }
    };
}

impl_115!();