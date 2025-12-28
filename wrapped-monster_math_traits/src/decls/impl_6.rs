macro_rules! deps {
    () => {
        DummySemanticHasher!();
        SemanticHasher!();
        Declaration!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl SemanticHasher for DummySemanticHasher { fn compute_semantic_hash (& self , mut declaration : Declaration) -> Declaration { declaration . semantic_hash = Some (format ! ("hash_{}" , declaration . name)) ; declaration . monster_factors = Some (vec ! [1 , 2 , 3]) ; declaration } }
    };
}

impl_6!()