macro_rules! deps {
    () => {
        BadDeriveTarget!();
    };
}

macro_rules! report_bad_target {
    () => {
        deps!();
        fn report_bad_target (sess : & Session , item : & Annotatable , span : Span ,) -> Result < () , ErrorGuaranteed > { let item_kind = match item { Annotatable :: Item (item) => Some (& item . kind) , Annotatable :: Stmt (stmt) => match & stmt . kind { StmtKind :: Item (item) => Some (& item . kind) , _ => None , } , _ => None , } ; let bad_target = ! matches ! (item_kind , Some (ItemKind :: Struct (..) | ItemKind :: Enum (..) | ItemKind :: Union (..))) ; if bad_target { return Err (sess . dcx () . emit_err (errors :: BadDeriveTarget { span , item : item . span () })) ; } Ok (()) }
    };
}

report_bad_target!();