macro_rules! deps {
    () => {
        ContentMerge!();
    };
}

macro_rules! ResolutionFailure {
    () => {
        deps!();
        # [doc = " Describes of a conflict involving *our* change and *their* failed to be resolved."] # [derive (Debug , Clone)] pub enum ResolutionFailure { # [doc = " *ours* was renamed, but *theirs* was renamed differently. Both versions will be present in the tree,"] OursRenamedTheirsRenamedDifferently { # [doc = " If `Some(…)`, the content of the involved blob had to be merged."] merged_blob : Option < ContentMerge > , } , # [doc = " *ours* was modified, but *theirs* was turned into a directory, so *ours* was renamed to a non-conflicting path."] OursModifiedTheirsDirectoryThenOursRenamed { # [doc = " The path at which `ours` can be found in the tree - it's in the same directory that it was in before."] renamed_unique_path_to_modified_blob : BString , } , # [doc = " *ours* is a directory, but *theirs* is a non-directory (i.e. file), which wants to be in its place, even though"] # [doc = " *ours* has a modification in that subtree."] # [doc = " Rename *theirs* to retain that modification."] # [doc = ""] # [doc = " Important: there is no actual modification on *ours* side, so *ours* is filled in with *theirs* as the data structure"] # [doc = " cannot represent this case."] OursDirectoryTheirsNonDirectoryTheirsRenamed { # [doc = " The non-conflicting path of *their* non-tree entry."] renamed_unique_path_of_theirs : BString , } , # [doc = " *ours* was added (or renamed into place) with a different mode than theirs, e.g. blob and symlink, and we kept"] # [doc = " the symlink in its original location, renaming the other side to `their_unique_location`."] OursAddedTheirsAddedTypeMismatch { # [doc = " The location at which *their* state was placed to resolve the name and type clash, named to indicate"] # [doc = " where the entry is coming from."] their_unique_location : BString , } , # [doc = " *ours* was modified, and they renamed the same file, but there is also a non-mergable type-change."] # [doc = " Here we keep both versions of the file."] OursModifiedTheirsRenamedTypeMismatch , # [doc = " *ours* was deleted, but *theirs* was renamed."] OursDeletedTheirsRenamed , # [doc = " *ours* was modified and *theirs* was deleted. We keep the modified one and ignore the deletion."] OursModifiedTheirsDeleted , # [doc = " *ours* and *theirs* are in an untested state so it can't be handled yet, and is considered a conflict"] # [doc = " without adding our *or* their side to the resulting tree."] Unknown , }
    };
}

ResolutionFailure!();