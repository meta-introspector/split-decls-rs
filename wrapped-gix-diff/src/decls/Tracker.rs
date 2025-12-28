macro_rules! deps {
    () => {
        Item!();
        Rewrites!();
        ChangeId!();
    };
}

macro_rules! Tracker {
    () => {
        deps!();
        # [doc = " A type to retain state related to an ongoing tracking operation to retain sets of interesting changes"] # [doc = " of which some are retained to at a later stage compute the ones that seem to be renames or copies."] pub struct Tracker < T > { # [doc = " The tracked items thus far, which will be used to determine renames/copies and rewrites later."] items : Vec < tracker :: Item < T > > , # [doc = " A place to store all paths in to reduce amount of allocations."] path_backing : Vec < u8 > , # [doc = " How to track copies and/or rewrites."] rewrites : Rewrites , # [doc = " Previously emitted relation ids of rewrite pairs, with `(deleted source, added destination)`."] child_renames : BTreeSet < (ChangeId , ChangeId) > , }
    };
}

Tracker!()