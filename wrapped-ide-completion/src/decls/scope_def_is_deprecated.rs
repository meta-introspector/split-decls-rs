macro_rules! deps {
    () => {
        RenderContext!();
    };
}

macro_rules! scope_def_is_deprecated {
    () => {
        deps!();
        fn scope_def_is_deprecated (ctx : & RenderContext < '_ > , resolution : ScopeDef) -> bool { match resolution { ScopeDef :: ModuleDef (it) => ctx . is_deprecated_assoc_item (it) , ScopeDef :: GenericParam (it) => ctx . is_deprecated (it) , ScopeDef :: AdtSelfType (it) => ctx . is_deprecated (it) , _ => false , } }
    };
}

scope_def_is_deprecated!()