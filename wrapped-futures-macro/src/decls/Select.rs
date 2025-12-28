macro_rules! Select {
    () => {
        struct Select { complete : Option < Expr > , default : Option < Expr > , normal_fut_exprs : Vec < Expr > , normal_fut_handlers : Vec < (Pat , Expr) > , }
    };
}

Select!();