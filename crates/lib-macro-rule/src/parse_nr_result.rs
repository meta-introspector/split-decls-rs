#[derive(Clone, Debug)]
pub enum ParseNtResult {
    Tt(TokenTree),
    Ident(Ident, IdentIsRaw),
    Lifetime(Ident, IdentIsRaw),
    Item(Box<Item>),
    Block(Block),
    Stmt(Stmt),
    Pat(Box<Pat>, NonterminalKind),
    Expr(Box<Expr>, NonterminalKind),
    Literal(Lit),
    Ty(Box<Ty>),
    Meta(MetaItem),
    Path(Path),
    Vis(Visibility),
}
