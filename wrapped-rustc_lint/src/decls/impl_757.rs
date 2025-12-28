macro_rules! deps {
    () => {
        EarlyContext!();
    };
}

macro_rules! impl_757 {
    () => {
        deps!();
        impl EarlyLintPass for RedundantSemicolons { fn check_block (& mut self , cx : & EarlyContext < '_ > , block : & Block) { let mut seq = None ; for stmt in block . stmts . iter () { match (& stmt . kind , & mut seq) { (StmtKind :: Empty , None) => seq = Some ((stmt . span , false)) , (StmtKind :: Empty , Some (seq)) => * seq = (seq . 0 . to (stmt . span) , true) , (_ , seq) => maybe_lint_redundant_semis (cx , seq) , } } maybe_lint_redundant_semis (cx , & mut seq) ; } }
    };
}

impl_757!();