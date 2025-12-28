macro_rules! deps {
    () => {
        Walkable!();
        Expr!();
    };
}

macro_rules! YieldKind {
    () => {
        deps!();
        # [doc = " The kind of yield expression"] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum YieldKind { # [doc = " yield expr { ... }"] Prefix (Option < Box < Expr > >) , # [doc = " expr.yield { ... }"] Postfix (Box < Expr >) , }
    };
}

YieldKind!()