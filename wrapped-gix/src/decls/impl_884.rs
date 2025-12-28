macro_rules! deps {
    () => {
        Kind!();
        Delegate!();
        ObjectKindHint!();
    };
}

macro_rules! impl_884 {
    () => {
        deps!();
        impl delegate :: Kind for Delegate < '_ > { fn kind (& mut self , kind : gix_revision :: spec :: Kind) -> Option < () > { use gix_revision :: spec :: Kind :: * ; self . kind = Some (kind) ; if self . kind_implies_committish () { self . disambiguate_objects_by_fallback_hint (ObjectKindHint :: Committish . into ()) ; } if matches ! (kind , RangeBetween | ReachableToMergeBase) { self . idx += 1 ; } Some (()) } }
    };
}

impl_884!()