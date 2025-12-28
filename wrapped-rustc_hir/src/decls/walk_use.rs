macro_rules! deps {
    () => {
        UsePath!();
        Path!();
        Visitor!();
    };
}

macro_rules! walk_use {
    () => {
        deps!();
        pub fn walk_use < 'v , V : Visitor < 'v > > (visitor : & mut V , path : & 'v UsePath < 'v > , hir_id : HirId ,) -> V :: Result { let UsePath { segments , ref res , span } = * path ; for res in res . present_items () { try_visit ! (visitor . visit_path (& Path { segments , res , span } , hir_id)) ; } V :: Result :: output () }
    };
}

walk_use!();