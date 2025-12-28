macro_rules! deps {
    () => {
        ObjectKindHint!();
        Delegate!();
    };
}

macro_rules! impl_883 {
    () => {
        deps!();
        impl parse :: Delegate for Delegate < '_ > { fn done (& mut self) { self . follow_refs_to_objects_if_needed () ; self . disambiguate_objects_by_fallback_hint (self . kind_implies_committish () . then_some (ObjectKindHint :: Committish) . or (self . opts . object_kind_hint) ,) ; } }
    };
}

impl_883!()