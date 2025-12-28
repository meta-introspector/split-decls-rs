macro_rules! deps {
    () => {
        ReferencedStatementsVisitor!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'v > Visitor < 'v > for ReferencedStatementsVisitor < '_ > { type Result = ControlFlow < () > ; fn visit_stmt (& mut self , s : & 'v hir :: Stmt < 'v >) -> Self :: Result { match s . kind { hir :: StmtKind :: Semi (expr) if self . 0 . contains (& expr . span) => ControlFlow :: Break (()) , _ => ControlFlow :: Continue (()) , } } }
    };
}

impl_118!();