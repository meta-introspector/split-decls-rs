macro_rules! deps {
    () => {
        Lifetime!();
        Item!();
        Expr!();
        NtPatKind!();
        NtExprKind!();
        Stmt!();
        Block!();
        Ty!();
        Path!();
        Pat!();
    };
}

macro_rules! NonterminalKind {
    () => {
        deps!();
        # [doc = " A macro nonterminal, known in documentation as a fragment specifier."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , HashStable_Generic)] pub enum NonterminalKind { Item , Block , Stmt , Pat (NtPatKind) , Expr (NtExprKind) , Ty , Ident , Lifetime , Literal , Meta , Path , Vis , TT , }
    };
}

NonterminalKind!()