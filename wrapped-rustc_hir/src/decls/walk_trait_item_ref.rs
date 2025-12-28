macro_rules! deps {
    () => {
        Visitor!();
        TraitItemId!();
    };
}

macro_rules! walk_trait_item_ref {
    () => {
        deps!();
        pub fn walk_trait_item_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , id : TraitItemId) -> V :: Result { visitor . visit_nested_trait_item (id) }
    };
}

walk_trait_item_ref!();