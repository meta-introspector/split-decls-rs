macro_rules! deps {
    () => {
        PatternID!();
        Span!();
    };
}

macro_rules! Match {
    () => {
        deps!();
        # [doc = " A representation of a match reported by a regex engine."] # [doc = ""] # [doc = " A match has two essential pieces of information: the [`PatternID`] that"] # [doc = " matches, and the [`Span`] of the match in a haystack."] # [doc = ""] # [doc = " The pattern is identified by an ID, which corresponds to its position"] # [doc = " (starting from `0`) relative to other patterns used to construct the"] # [doc = " corresponding regex engine. If only a single pattern is provided, then all"] # [doc = " matches are guaranteed to have a pattern ID of `0`."] # [doc = ""] # [doc = " Every match reported by a regex engine guarantees that its span has its"] # [doc = " start offset as less than or equal to its end offset."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct Match { # [doc = " The pattern ID."] pattern : PatternID , # [doc = " The underlying match span."] span : Span , }
    };
}

Match!()