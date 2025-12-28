macro_rules! deps {
    () => {
        Schema!();
        BoxFieldFuture!();
        Field!();
        ContextSelectionSet!();
        OutputType!();
    };
}

macro_rules! collect_schema_field {
    () => {
        deps!();
        fn collect_schema_field < 'a > (fields : & mut Vec < BoxFieldFuture < 'a > > , ctx : & ContextSelectionSet < 'a > , field : & 'a Positioned < Field > ,) { let ctx = ctx . clone () ; fields . push (async move { let ctx_field = ctx . with_field (field) ; let mut ctx_obj = ctx . with_selection_set (& ctx_field . item . node . selection_set) ; ctx_obj . is_for_introspection = true ; let visible_types = ctx . schema_env . registry . find_visible_types (& ctx_field) ; let value = crate :: OutputType :: resolve (& crate :: model :: __Schema :: new (& ctx . schema_env . registry , & visible_types) , & ctx_obj , ctx_field . item ,) . await ? ; Ok ((field . node . response_key () . node . clone () , value)) } . boxed () ,) ; }
    };
}

collect_schema_field!();