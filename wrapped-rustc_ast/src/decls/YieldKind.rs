macro_rules! deps {
    () => {
        Expr!();
        Walkable!();
    };
}

macro_rules! YieldKind {
    () => {
        deps!();
        # [doc = " The kind of yield expression"] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum YieldKind { # [doc = " yield expr { ... }"] Prefix (Option < Box < Expr > >) , # [doc = " expr.yield { ... }"] Postfix (Box < Expr >) , }
    };
}

YieldKind!();