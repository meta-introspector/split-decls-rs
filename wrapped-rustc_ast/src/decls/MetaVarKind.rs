macro_rules! deps {
    () => {
        Item!();
        Expr!();
        Ty!();
        Pat!();
        Block!();
        Stmt!();
        NtExprKind!();
        NonterminalKind!();
        NtPatKind!();
        Path!();
        Lifetime!();
    };
}

macro_rules! MetaVarKind {
    () => {
        deps!();
        # [doc = " Annoyingly similar to `NonterminalKind`, but the slight differences are important."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Encodable , Decodable , Hash , HashStable_Generic)] pub enum MetaVarKind { Item , Block , Stmt , Pat (NtPatKind) , Expr { kind : NtExprKind , can_begin_literal_maybe_minus : bool , can_begin_string_literal : bool , } , Ty { is_path : bool , } , Ident , Lifetime , Literal , Meta { # [doc = " Will `AttrItem::meta` succeed on this, if reparsed?"] has_meta_form : bool , } , Path , Vis , TT , }
    };
}

MetaVarKind!();