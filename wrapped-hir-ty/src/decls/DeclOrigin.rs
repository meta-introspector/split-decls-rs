macro_rules! DeclOrigin {
    () => {
        # [derive (Debug , Copy , Clone)] pub (crate) enum DeclOrigin { LetExpr , # [doc = " from `let x = ..`"] LocalDecl { has_else : bool , } , }
    };
}

DeclOrigin!();