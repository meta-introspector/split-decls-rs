macro_rules! CaseKind {
    () => {
        # [allow (clippy :: large_enum_variant)] enum CaseKind { Complete , Default , Normal (Pat , Expr) , }
    };
}

CaseKind!()