macro_rules! deps {
    () => {
        DropGlue!();
        TraitEnvironment!();
    };
}

macro_rules! has_drop_glue {
    () => {
        deps!();
        pub fn has_drop_glue < 'db > (infcx : & InferCtxt < 'db > , ty : Ty < 'db > , env : Arc < TraitEnvironment < 'db > > ,) -> DropGlue { has_drop_glue_impl (infcx , ty , env , & mut FxHashSet :: default ()) }
    };
}

has_drop_glue!()