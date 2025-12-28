macro_rules! deps {
    () => {
        Visitor!();
    };
}

macro_rules! walk_ident {
    () => {
        deps!();
        pub fn walk_ident < 'v , V : Visitor < 'v > > (visitor : & mut V , ident : Ident) -> V :: Result { visitor . visit_name (ident . name) }
    };
}

walk_ident!()