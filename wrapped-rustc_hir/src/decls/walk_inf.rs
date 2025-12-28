macro_rules! deps {
    () => {
        InferArg!();
        Visitor!();
    };
}

macro_rules! walk_inf {
    () => {
        deps!();
        pub fn walk_inf < 'v , V : Visitor < 'v > > (visitor : & mut V , inf : & 'v InferArg) -> V :: Result { let InferArg { hir_id , span : _ } = inf ; visitor . visit_id (* hir_id) }
    };
}

walk_inf!();