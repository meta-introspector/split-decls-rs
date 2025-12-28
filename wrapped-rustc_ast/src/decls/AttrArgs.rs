macro_rules! deps {
    () => {
        Expr!();
        DelimArgs!();
        Walkable!();
    };
}

macro_rules! AttrArgs {
    () => {
        deps!();
        # [doc = " Arguments passed to an attribute macro."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum AttrArgs { # [doc = " No arguments: `#[attr]`."] Empty , # [doc = " Delimited arguments: `#[attr()/[]/{}]`."] Delimited (DelimArgs) , # [doc = " Arguments of a key-value attribute: `#[attr = \"value\"]`."] Eq { # [doc = " Span of the `=` token."] eq_span : Span , expr : Box < Expr > , } , }
    };
}

AttrArgs!();