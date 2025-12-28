macro_rules! deps {
    () => {
        UseTree!();
        Walkable!();
    };
}

macro_rules! UseTreeKind {
    () => {
        deps!();
        # [doc = " Part of `use` item to the right of its prefix."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum UseTreeKind { # [doc = " `use prefix` or `use prefix as rename`"] Simple (Option < Ident >) , # [doc = " `use prefix::{...}`"] # [doc = ""] # [doc = " The span represents the braces of the nested group and all elements within:"] # [doc = ""] # [doc = " ```text"] # [doc = " use foo::{bar, baz};"] # [doc = "          ^^^^^^^^^^"] # [doc = " ```"] Nested { items : ThinVec < (UseTree , NodeId) > , span : Span } , # [doc = " `use prefix::*`"] Glob , }
    };
}

UseTreeKind!();