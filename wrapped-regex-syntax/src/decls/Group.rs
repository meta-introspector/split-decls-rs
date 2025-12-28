macro_rules! deps {
    () => {
        Ast!();
        Span!();
        GroupKind!();
    };
}

macro_rules! Group {
    () => {
        deps!();
        # [doc = " A grouped regular expression."] # [doc = ""] # [doc = " This includes both capturing and non-capturing groups. This does **not**"] # [doc = " include flag-only groups like `(?is)`, but does contain any group that"] # [doc = " contains a sub-expression, e.g., `(a)`, `(?P<name>a)`, `(?:a)` and"] # [doc = " `(?is:a)`."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Group { # [doc = " The span of this group."] pub span : Span , # [doc = " The kind of this group."] pub kind : GroupKind , # [doc = " The regular expression in this group."] pub ast : Box < Ast > , }
    };
}

Group!();