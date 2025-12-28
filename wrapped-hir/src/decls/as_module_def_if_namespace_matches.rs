macro_rules! deps {
    () => {
        DocLinkDef!();
        TypeAlias!();
        Function!();
        Const!();
        AssocItem!();
        ModuleDef!();
    };
}

macro_rules! as_module_def_if_namespace_matches {
    () => {
        deps!();
        fn as_module_def_if_namespace_matches (assoc_item : AssocItem , ns : Option < Namespace > ,) -> Option < DocLinkDef > { let (def , expected_ns) = match assoc_item { AssocItem :: Function (it) => (ModuleDef :: Function (it) , Namespace :: Values) , AssocItem :: Const (it) => (ModuleDef :: Const (it) , Namespace :: Values) , AssocItem :: TypeAlias (it) => (ModuleDef :: TypeAlias (it) , Namespace :: Types) , } ; (ns . unwrap_or (expected_ns) == expected_ns) . then_some (DocLinkDef :: ModuleDef (def)) }
    };
}

as_module_def_if_namespace_matches!();