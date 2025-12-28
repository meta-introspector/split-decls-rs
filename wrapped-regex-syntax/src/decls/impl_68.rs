macro_rules! deps {
    () => {
        Ast!();
        Alternation!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl Alternation { # [doc = " Return this alternation as an AST."] # [doc = ""] # [doc = " If this alternation contains zero ASTs, then `Ast::empty` is returned."] # [doc = " If this alternation contains exactly 1 AST, then the corresponding AST"] # [doc = " is returned. Otherwise, `Ast::alternation` is returned."] pub fn into_ast (mut self) -> Ast { match self . asts . len () { 0 => Ast :: empty (self . span) , 1 => self . asts . pop () . unwrap () , _ => Ast :: alternation (self) , } } }
    };
}

impl_68!();