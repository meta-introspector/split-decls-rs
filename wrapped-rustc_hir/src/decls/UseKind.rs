macro_rules! UseKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug , HashStable_Generic)] pub enum UseKind { # [doc = " One import, e.g., `use foo::bar` or `use foo::bar as baz`."] # [doc = " Also produced for each element of a list `use`, e.g."] # [doc = " `use foo::{a, b}` lowers to `use foo::a; use foo::b;`."] # [doc = ""] # [doc = " The identifier is the name defined by the import. E.g. for `use"] # [doc = " foo::bar` it is `bar`, for `use foo::bar as baz` it is `baz`."] Single (Ident) , # [doc = " Glob import, e.g., `use foo::*`."] Glob , # [doc = " Degenerate list import, e.g., `use foo::{a, b}` produces"] # [doc = " an additional `use foo::{}` for performing checks such as"] # [doc = " unstable feature gating. May be removed in the future."] ListStem , }
    };
}

UseKind!()