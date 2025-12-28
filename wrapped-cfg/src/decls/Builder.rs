macro_rules! deps {
    () => {
        DnfExpr!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        struct Builder { expr : DnfExpr , }
    };
}

Builder!()