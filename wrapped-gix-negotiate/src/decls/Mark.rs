macro_rules! Mark {
    () => {
        enum Mark { AncestorsOnly , ThisCommitAndAncestors , }
    };
}

Mark!();