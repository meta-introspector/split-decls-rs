macro_rules! deps {
    () => {
        Visitor!();
        Lifetime!();
    };
}

macro_rules! walk_lifetime {
    () => {
        deps!();
        pub fn walk_lifetime < 'v , V : Visitor < 'v > > (visitor : & mut V , lifetime : & 'v Lifetime) -> V :: Result { let Lifetime { hir_id , ident , kind : _ , source : _ , syntax : _ } = lifetime ; try_visit ! (visitor . visit_id (* hir_id)) ; visitor . visit_ident (* ident) }
    };
}

walk_lifetime!();