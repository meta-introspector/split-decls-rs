macro_rules! deps {
    () => {
        VariantDef!();
        DocLinkDef!();
        Field!();
    };
}

macro_rules! resolve_field {
    () => {
        deps!();
        fn resolve_field (db : & dyn HirDatabase , def : VariantDef , name : Name , ns : Option < Namespace > ,) -> Option < DocLinkDef > { if let Some (Namespace :: Types | Namespace :: Macros) = ns { return None ; } def . fields (db) . into_iter () . find (| f | f . name (db) == name) . map (DocLinkDef :: Field) }
    };
}

resolve_field!();