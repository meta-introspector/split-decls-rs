macro_rules! sealed {
    () => {
        mod sealed { use rustc_ast_ir :: visit :: VisitorResult ; # [doc = " This is for compatibility with the regular `Visitor`."] pub trait MutVisitorResult { type Result : VisitorResult ; } impl < T > MutVisitorResult for T { type Result = () ; } }
    };
}

sealed!()