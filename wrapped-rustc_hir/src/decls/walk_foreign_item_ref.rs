macro_rules! deps {
    () => {
        Visitor!();
        ForeignItemId!();
    };
}

macro_rules! walk_foreign_item_ref {
    () => {
        deps!();
        pub fn walk_foreign_item_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , id : ForeignItemId) -> V :: Result { visitor . visit_nested_foreign_item (id) }
    };
}

walk_foreign_item_ref!()