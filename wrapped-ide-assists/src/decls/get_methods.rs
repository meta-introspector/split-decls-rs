macro_rules! get_methods {
    () => {
        pub (crate) fn get_methods (items : & ast :: AssocItemList) -> Vec < ast :: Fn > { items . assoc_items () . flat_map (| i | match i { ast :: AssocItem :: Fn (f) => Some (f) , _ => None , }) . filter (| f | f . name () . is_some ()) . collect () }
    };
}

get_methods!()