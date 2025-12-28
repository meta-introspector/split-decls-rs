macro_rules! deps {
    () => {
        Mod!();
        Visitor!();
    };
}

macro_rules! walk_mod {
    () => {
        deps!();
        pub fn walk_mod < 'v , V : Visitor < 'v > > (visitor : & mut V , module : & 'v Mod < 'v >) -> V :: Result { let Mod { spans : _ , item_ids } = module ; walk_list ! (visitor , visit_nested_item , item_ids . iter () . copied ()) ; V :: Result :: output () }
    };
}

walk_mod!()