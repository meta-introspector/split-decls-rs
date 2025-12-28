macro_rules! deps {
    () => {
        Clone!();
        Commits!();
        Default!();
        Note!();
    };
}

macro_rules! Sorting {
    () => {
        deps!();
        # [doc = " Specify how to sort commits during a [revision::Walk] traversal."] # [doc = ""] # [doc = " ### Sample History"] # [doc = ""] # [doc = " The following history will be referred to for explaining how the sort order works, with the number denoting the commit timestamp"] # [doc = " (*their X-alignment doesn't matter*)."] # [doc = ""] # [doc = " ```text"] # [doc = " ---1----2----4----7 <- second parent of 8"] # [doc = "     \\              \\"] # [doc = "      3----5----6----8---"] # [doc = " ```"] # [derive (Default , Debug , Copy , Clone)] pub enum Sorting { # [doc = " Commits are sorted as they are mentioned in the commit graph."] # [doc = ""] # [doc = " In the *sample history* the order would be `8, 6, 7, 5, 4, 3, 2, 1`"] # [doc = ""] # [doc = " ### Note"] # [doc = ""] # [doc = " This is not to be confused with `git log/rev-list --topo-order`, which is notably different from"] # [doc = " as it avoids overlapping branches."] # [default] BreadthFirst , # [doc = " Commits are sorted by their commit time in the order specified, either newest or oldest first."] # [doc = ""] # [doc = " The sorting applies to all currently queued commit ids and thus is full."] # [doc = ""] # [doc = " In the *sample history* the order would be `8, 7, 6, 5, 4, 3, 2, 1` for [`NewestFirst`](CommitTimeOrder::NewestFirst),"] # [doc = " or `1, 2, 3, 4, 5, 6, 7, 8` for [`OldestFirst`](CommitTimeOrder::OldestFirst)."] # [doc = ""] # [doc = " # Performance"] # [doc = ""] # [doc = " This mode benefits greatly from having an [object cache](crate::Repository::object_cache_size) configured"] # [doc = " to avoid having to look up each commit twice."] ByCommitTime (CommitTimeOrder) , # [doc = " This sorting is similar to [`ByCommitTime`](Sorting::ByCommitTimeCutoff), but adds a cutoff to not return commits older than"] # [doc = " a given time, stopping the iteration once no younger commits is queued to be traversed."] # [doc = ""] # [doc = " As the query is usually repeated with different cutoff dates, this search mode benefits greatly from an object cache."] # [doc = ""] # [doc = " In the *sample history* and a cut-off date of 4, the returned list of commits would be `8, 7, 6, 4`"] ByCommitTimeCutoff { # [doc = " The order in wich to prioritize lookups"] order : CommitTimeOrder , # [doc = " The amount of seconds since unix epoch to use as cut-off time."] seconds : gix_date :: SecondsSinceUnixEpoch , } , }
    };
}

Sorting!();