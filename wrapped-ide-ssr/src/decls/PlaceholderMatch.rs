macro_rules! deps {
    () => {
        SsrMatches!();
    };
}

macro_rules! PlaceholderMatch {
    () => {
        deps!();
        # [doc = " Information about a placeholder bound in a match."] # [derive (Debug)] pub (crate) struct PlaceholderMatch { pub (crate) range : FileRange , # [doc = " More matches, found within `node`."] pub (crate) inner_matches : SsrMatches , # [doc = " How many times the code that the placeholder matched needed to be dereferenced. Will only be"] # [doc = " non-zero if the placeholder matched to the receiver of a method call."] pub (crate) autoderef_count : usize , pub (crate) autoref_kind : ast :: SelfParamKind , }
    };
}

PlaceholderMatch!();