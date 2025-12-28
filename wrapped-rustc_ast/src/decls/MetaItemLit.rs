macro_rules! deps {
    () => {
        LitKind!();
    };
}

macro_rules! MetaItemLit {
    () => {
        deps!();
        # [doc = " A literal in a meta item."] # [derive (Clone , Copy , Encodable , Decodable , Debug , HashStable_Generic)] pub struct MetaItemLit { # [doc = " The original literal as written in the source code."] pub symbol : Symbol , # [doc = " The original suffix as written in the source code."] pub suffix : Option < Symbol > , # [doc = " The \"semantic\" representation of the literal lowered from the original tokens."] # [doc = " Strings are unescaped, hexadecimal forms are eliminated, etc."] pub kind : LitKind , pub span : Span , }
    };
}

MetaItemLit!();