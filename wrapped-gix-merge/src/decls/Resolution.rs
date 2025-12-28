macro_rules! deps {
    () => {
        ContentMerge!();
        ResolutionFailure!();
    };
}

macro_rules! Resolution {
    () => {
        deps!();
        # [doc = " Describes of a conflict involving *our* change and *their* change was specifically resolved."] # [doc = ""] # [doc = " Note that all resolutions are side-agnostic, so *ours* could also have been *theirs* and vice versa."] # [doc = " Also note that symlink merges are always done via binary merge, using the same logic."] # [derive (Debug , Clone)] pub enum Resolution { # [doc = " *ours* had a renamed directory and *theirs* made a change in the now renamed directory."] # [doc = " We moved that change into its location."] SourceLocationAffectedByRename { # [doc = " The repository-relative path to the location that the change ended up in after"] # [doc = " being affected by a renamed directory."] final_location : BString , } , # [doc = " *ours* was a modified blob and *theirs* renamed that blob."] # [doc = " We moved the changed blob from *ours* to its new location, and merged it successfully."] # [doc = " If this is a `copy`, the source of the copy was set to be the changed blob as well so both match."] OursModifiedTheirsRenamedAndChangedThenRename { # [doc = " If one side added the executable bit, we always add it in the merged result."] merged_mode : Option < gix_object :: tree :: EntryMode > , # [doc = " If `Some(…)`, the content of the involved blob had to be merged."] merged_blob : Option < ContentMerge > , # [doc = " The repository relative path to the location the blob finally ended up in."] # [doc = " It's `Some()` only if *they* rewrote the blob into a directory which *we* renamed on *our* side."] final_location : Option < BString > , } , # [doc = " *ours* and *theirs* carried changes and where content-merged."] # [doc = ""] # [doc = " Note that *ours* and *theirs* may also be rewrites with the same destination and mode,"] # [doc = " or additions."] OursModifiedTheirsModifiedThenBlobContentMerge { # [doc = " The outcome of the content merge."] merged_blob : ContentMerge , } , # [doc = " This is a resolution failure was forcefully turned into a usable resolution, i.e. [making a choice](ResolveWith)"] # [doc = " is turned into a valid resolution."] Forced (ResolutionFailure) , }
    };
}

Resolution!();