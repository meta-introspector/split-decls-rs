macro_rules! deps {
    () => {
        Visitor!();
    };
}

macro_rules! walk_label {
    () => {
        deps!();
        pub fn walk_label < 'v , V : Visitor < 'v > > (visitor : & mut V , label : & 'v Label) -> V :: Result { let Label { ident } = label ; visitor . visit_ident (* ident) }
    };
}

walk_label!()