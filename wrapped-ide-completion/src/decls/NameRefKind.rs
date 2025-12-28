macro_rules! deps {
    () => {
        PatternContext!();
        PathCompletionCtx!();
        DotAccess!();
    };
}

macro_rules! NameRefKind {
    () => {
        deps!();
        # [doc = " The kind of the NameRef we are completing."] # [derive (Debug)] pub (crate) enum NameRefKind < 'db > { Path (PathCompletionCtx < 'db >) , DotAccess (DotAccess < 'db >) , # [doc = " Position where we are only interested in keyword completions"] Keyword (ast :: Item) , # [doc = " The record expression this nameref is a field of and whether a dot precedes the completion identifier."] RecordExpr { dot_prefix : bool , expr : ast :: RecordExpr , } , Pattern (PatternContext) , ExternCrate , }
    };
}

NameRefKind!()