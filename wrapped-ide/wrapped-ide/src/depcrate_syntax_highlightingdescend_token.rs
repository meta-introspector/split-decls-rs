// Generated macro for descend_token (function)
macro_rules! Depcrate_syntax_highlightingdescend_token {
() => {
// Module: crate::syntax_highlighting
// Provides: {"descend_token"}
// Dependencies: {}
fn descend_token (sema : & Semantics < '_ , RootDatabase > , token : InRealFile < SyntaxToken > ,) -> InFile < NodeOrToken < ast :: NameLike , SyntaxToken > > { if token . value . kind () == COMMENT { return token . map (NodeOrToken :: Token) . into () ; } let ranker = Ranker :: from_token (& token . value) ; let mut t = None ; let mut r = 0 ; sema . descend_into_macros_breakable (token . clone () . into () , | tok , _ctx | { let my_rank = ranker . rank_token (& tok . value) ; if my_rank >= Ranker :: MAX_RANK { t = Some (tok) ; return ControlFlow :: Break (()) ; } match & mut t { Some (prev) if r < my_rank => { * prev = tok ; r = my_rank ; } Some (_) => () , None => { r = my_rank ; t = Some (tok) } } ControlFlow :: Continue (()) }) ; let token = t . unwrap_or_else (| | token . into ()) ; token . map (| token | match token . parent () . and_then (ast :: NameLike :: cast) { Some (parent) => match (token . kind () , parent . syntax () . kind ()) { (T ! [ident] | T ! [self] , NAME) | (T ! [ident] | T ! [self] | T ! [super] | T ! [crate] | T ! [Self] , NAME_REF) | (INT_NUMBER , NAME_REF) | (LIFETIME_IDENT , LIFETIME) => NodeOrToken :: Node (parent) , _ => NodeOrToken :: Token (token) , } , None => NodeOrToken :: Token (token) , }) }
};
}
