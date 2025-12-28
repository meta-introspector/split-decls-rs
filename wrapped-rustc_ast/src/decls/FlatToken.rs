macro_rules! deps {
    () => {
        AttrsTarget!();
        AttrTokenStream!();
        Token!();
        LazyAttrTokenStream!();
        Spacing!();
    };
}

macro_rules! FlatToken {
    () => {
        deps!();
        # [doc = " A helper struct used when building an `AttrTokenStream` from"] # [doc = " a `LazyAttrTokenStream`. Both delimiter and non-delimited tokens"] # [doc = " are stored as `FlatToken::Token`. A vector of `FlatToken`s"] # [doc = " is then 'parsed' to build up an `AttrTokenStream` with nested"] # [doc = " `AttrTokenTree::Delimited` tokens."] # [derive (Debug , Clone)] enum FlatToken { # [doc = " A token - this holds both delimiter (e.g. '{' and '}')"] # [doc = " and non-delimiter tokens"] Token ((Token , Spacing)) , # [doc = " Holds the `AttrsTarget` for an AST node. The `AttrsTarget` is inserted"] # [doc = " directly into the constructed `AttrTokenStream` as an"] # [doc = " `AttrTokenTree::AttrsTarget`."] AttrsTarget (AttrsTarget) , # [doc = " A special 'empty' token that is ignored during the conversion"] # [doc = " to an `AttrTokenStream`. This is used to simplify the"] # [doc = " handling of replace ranges."] Empty , }
    };
}

FlatToken!();