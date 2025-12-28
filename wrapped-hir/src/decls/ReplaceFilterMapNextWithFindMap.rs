macro_rules! ReplaceFilterMapNextWithFindMap {
    () => {
        # [derive (Debug)] pub struct ReplaceFilterMapNextWithFindMap { pub file : HirFileId , # [doc = " This expression is the whole method chain up to and including `.filter_map(..).next()`."] pub next_expr : AstPtr < ast :: Expr > , }
    };
}

ReplaceFilterMapNextWithFindMap!()