macro_rules! deps {
    () => {
        RemoveAttrsFromTraitMethods!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl VisitMut for RemoveAttrsFromTraitMethods { fn visit_item_trait_mut (& mut self , i : & mut syn :: ItemTrait) { for item in & mut i . items { if let TraitItem :: Fn (trait_item_fn) = item { trait_item_fn . attrs = vec ! [] ; } } } }
    };
}

impl_37!()