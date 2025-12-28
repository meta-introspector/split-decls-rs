macro_rules! SuffixKind {
    () => {
        # [doc = " The kind of suffix to extract."] # [derive (Clone , Copy , Debug)] enum SuffixKind { # [doc = " Extract the smallest lexicographic suffix from a string."] # [doc = ""] # [doc = " Technically, this doesn't actually pick the smallest lexicographic"] # [doc = " suffix. e.g., Given the choice between `a` and `aa`, this will choose"] # [doc = " the latter over the former, even though `a < aa`. The reasoning for"] # [doc = " this isn't clear from the paper, but it still smells like a minimal"] # [doc = " suffix."] Minimal , # [doc = " Extract the largest lexicographic suffix from a string."] # [doc = ""] # [doc = " Unlike `Minimal`, this really does pick the maximum suffix. e.g., Given"] # [doc = " the choice between `z` and `zz`, this will choose the latter over the"] # [doc = " former."] Maximal , }
    };
}

SuffixKind!()