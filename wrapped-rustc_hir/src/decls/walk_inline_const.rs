macro_rules! deps {
    () => {
        Visitor!();
        ConstBlock!();
    };
}

macro_rules! walk_inline_const {
    () => {
        deps!();
        pub fn walk_inline_const < 'v , V : Visitor < 'v > > (visitor : & mut V , constant : & 'v ConstBlock ,) -> V :: Result { let ConstBlock { hir_id , def_id : _ , body } = constant ; try_visit ! (visitor . visit_id (* hir_id)) ; visitor . visit_nested_body (* body) }
    };
}

walk_inline_const!();