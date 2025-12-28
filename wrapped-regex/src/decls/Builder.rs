macro_rules! deps {
    () => {
        RegexSet!();
        Regex!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " A builder for constructing a `Regex`, `bytes::Regex`, `RegexSet` or a"] # [doc = " `bytes::RegexSet`."] # [doc = ""] # [doc = " This is essentially the implementation of the four different builder types"] # [doc = " in the public API: `RegexBuilder`, `bytes::RegexBuilder`, `RegexSetBuilder`"] # [doc = " and `bytes::RegexSetBuilder`."] # [derive (Clone , Debug)] struct Builder { pats : Vec < String > , metac : meta :: Config , syntaxc : syntax :: Config , }
    };
}

Builder!();