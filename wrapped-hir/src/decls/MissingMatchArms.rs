macro_rules! MissingMatchArms {
    () => {
        # [derive (Debug)] pub struct MissingMatchArms { pub scrutinee_expr : InFile < AstPtr < ast :: Expr > > , pub uncovered_patterns : String , }
    };
}

MissingMatchArms!();