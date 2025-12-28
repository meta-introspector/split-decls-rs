macro_rules! deps {
    () => {
        EarlyContextAndPass!();
        EarlyContext!();
        EarlyCheckNode!();
    };
}

macro_rules! check_ast_node_inner {
    () => {
        deps!();
        fn check_ast_node_inner < 'a , T : EarlyLintPass > (sess : & Session , tcx : Option < TyCtxt < '_ > > , check_node : impl EarlyCheckNode < 'a > , context : EarlyContext < '_ > , pass : T ,) { let mut cx = EarlyContextAndPass { context , tcx , pass } ; cx . with_lint_attrs (check_node . id () , check_node . attrs () , | cx | check_node . check (cx)) ; for (id , lints) in cx . context . buffered . map { if ! lints . is_empty () { assert ! (sess . dcx () . has_errors () . is_some () , "failed to process buffered lint here (dummy = {})" , id == ast :: DUMMY_NODE_ID) ; break ; } } }
    };
}

check_ast_node_inner!()