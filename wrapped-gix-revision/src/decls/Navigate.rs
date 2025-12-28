macro_rules! deps {
    () => {
        Traversal!();
        PeelTo!();
    };
}

macro_rules! Navigate {
    () => {
        deps!();
        # [doc = " Once an anchor is set one can adjust it using traversal methods."] pub trait Navigate { # [doc = " Adjust the current revision to traverse the graph according to `kind`."] fn traverse (& mut self , kind : Traversal) -> Option < () > ; # [doc = " Peel the current object until it reached `kind` or `None` if the chain does not contain such object."] fn peel_until (& mut self , kind : PeelTo < '_ >) -> Option < () > ; # [doc = " Find the first revision/commit whose message matches the given `regex` (which is never empty)."] # [doc = " to see how it should be matched."] # [doc = " If `negated` is `true`, the first non-match will be a match."] # [doc = ""] # [doc = " If no revision is known yet, find the _youngest_ matching commit from _any_ reference, including `HEAD`."] # [doc = " Otherwise, only find commits reachable from the currently set revision."] fn find (& mut self , regex : & BStr , negated : bool) -> Option < () > ; # [doc = " Look up the given `path` at the given `stage` in the index returning its blob id,"] # [doc = " or return `None` if it doesn't exist at this `stage`."] # [doc = " Note that this implies no revision is needed and no anchor is set yet."] # [doc = ""] # [doc = " * `stage` ranges from 0 to 2, with 0 being the base, 1 being ours, 2 being theirs."] # [doc = " * `path` without prefix is relative to the root of the repository, while prefixes like `./` and `../` make it"] # [doc = "   relative to the current working directory."] fn index_lookup (& mut self , path : & BStr , stage : u8) -> Option < () > ; }
    };
}

Navigate!();