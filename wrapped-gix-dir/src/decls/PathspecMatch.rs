macro_rules! PathspecMatch {
    () => {
        # [doc = " Describe how a pathspec pattern matched."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub enum PathspecMatch { # [doc = " The match happened because there wasn't any pattern, which matches all, or because there was a nil pattern or one with an empty path."] # [doc = " Thus, this is not a match by merit."] Always , # [doc = " A match happened, but the pattern excludes everything it matches, which means this entry was excluded."] Excluded , # [doc = " The first part of a pathspec matches, like `dir/` that matches `dir/a`."] Prefix , # [doc = " The whole pathspec matched and used a wildcard match, like `a/*` matching `a/file`."] WildcardMatch , # [doc = " The entire pathspec matched, letter by letter, e.g. `a/file` matching `a/file`."] Verbatim , }
    };
}

PathspecMatch!()