macro_rules! deps {
    () => {
        Delegate!();
    };
}

macro_rules! CollapsedEntriesEmissionMode {
    () => {
        deps!();
        # [doc = " The way entries that are contained in collapsed directories are emitted using the [Delegate]."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq , Hash , Ord , PartialOrd)] pub enum CollapsedEntriesEmissionMode { # [doc = " Emit only entries if their status does not match the one of the parent directory that is"] # [doc = " going to be collapsed."] # [doc = ""] # [doc = " E.g. if a directory is determined to be untracked, and the entries in question are ignored,"] # [doc = " they will be emitted."] # [doc = ""] # [doc = " Entries that have the same status will essentially be 'merged' into the collapsing directory"] # [doc = " and won't be observable anymore."] # [default] OnStatusMismatch , # [doc = " Emit all entries inside of a collapsed directory to make them observable."] All , }
    };
}

CollapsedEntriesEmissionMode!()