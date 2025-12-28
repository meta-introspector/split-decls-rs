macro_rules! deps {
    () => {
        Visitor!();
        ImplItemId!();
    };
}

macro_rules! walk_impl_item_ref {
    () => {
        deps!();
        pub fn walk_impl_item_ref < 'v , V : Visitor < 'v > > (visitor : & mut V , id : ImplItemId) -> V :: Result { visitor . visit_nested_impl_item (id) }
    };
}

walk_impl_item_ref!()