macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! LineColumn {
    () => {
        deps!();
        # [doc = " A line-column pair representing the start or end of a `Span`."] # [doc = ""] # [doc = " This type is semver exempt and not exposed by default."] # [cfg_attr (docsrs , doc (cfg (feature = "span-locations")))] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub struct LineColumn { # [doc = " The 1-indexed line in the source file on which the span starts or ends"] # [doc = " (inclusive)."] pub line : usize , # [doc = " The 0-indexed column (in UTF-8 characters) in the source file on which"] # [doc = " the span starts or ends (inclusive)."] pub column : usize , }
    };
}

LineColumn!()