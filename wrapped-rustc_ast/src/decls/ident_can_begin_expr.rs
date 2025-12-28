macro_rules! deps {
    () => {
        IdentIsRaw!();
        Token!();
        Const!();
    };
}

macro_rules! ident_can_begin_expr {
    () => {
        deps!();
        pub fn ident_can_begin_expr (name : Symbol , span : Span , is_raw : IdentIsRaw) -> bool { let ident_token = Token :: new (Ident (name , is_raw) , span) ; ! ident_token . is_reserved_ident () || ident_token . is_path_segment_keyword () || [kw :: Async , kw :: Do , kw :: Box , kw :: Break , kw :: Const , kw :: Continue , kw :: False , kw :: For , kw :: Gen , kw :: If , kw :: Let , kw :: Loop , kw :: Match , kw :: Move , kw :: Return , kw :: True , kw :: Try , kw :: Unsafe , kw :: While , kw :: Yield , kw :: Safe , kw :: Static ,] . contains (& name) }
    };
}

ident_can_begin_expr!();