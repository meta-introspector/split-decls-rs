macro_rules! deps {
    () => {
        Change!();
        ConflictIndexEntry!();
        Conflict!();
    };
}

macro_rules! EntryStatus {
    () => {
        deps!();
        # [doc = " Information about an entry."] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub enum EntryStatus < T = () , U = () > { # [doc = " The entry is in a conflicting state, and we provide all related entries along with a summary."] Conflict { # [doc = " An analysis on the conflict itself based on the observed index entries."] summary : Conflict , # [doc = " The entries from stage 1 to stage 3, where stage 1 is at index 0 and stage 3 at index 2."] # [doc = " Note that when there are conflicts, there is no stage 0."] # [doc = " Further, all entries are looking at the same path."] entries : Box < [Option < ConflictIndexEntry > ; 3] > , } , # [doc = " There is no conflict and a change was discovered."] Change (Change < T , U >) , # [doc = " The entry didn't change, but its state caused extra work that can be avoided next time if its stats would be updated to the"] # [doc = " given stat."] NeedsUpdate (# [doc = " The new stats which represent what's currently in the working tree. If these replace the current stats in the entry,"] # [doc = " next time this operation runs we can determine the actual state much faster."] gix_index :: entry :: Stat ,) , # [doc = " An index entry that corresponds to an untracked worktree file marked with `git add --intent-to-add`."] # [doc = ""] # [doc = " This means it's not available in the object database yet even though now an entry exists that represents the worktree file."] # [doc = " The entry represents the promise of adding a new file, no matter the actual stat or content."] # [doc = " Effectively this means nothing changed."] # [doc = " This also means the file is still present, and that no detailed change checks were performed."] IntentToAdd , }
    };
}

EntryStatus!()