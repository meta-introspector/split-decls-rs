macro_rules! deps {
    () => {
        Resolution!();
        ConflictIndexEntry!();
        ResolutionFailure!();
        ConflictMapping!();
    };
}

macro_rules! Conflict {
    () => {
        deps!();
        # [doc = " A description of a conflict (i.e. merge issue without an auto-resolution) as seen during a [tree-merge](crate::tree())."] # [doc = " They may have a resolution that was applied automatically, or be left for the caller to resolved."] # [derive (Debug , Clone)] pub struct Conflict { # [doc = " A record on how the conflict resolution succeeded with `Ok(_)` or failed with `Err(_)`."] # [doc = " Note that in case of `Err(_)`, edits may still have been made to the tree to aid resolution."] # [doc = " On failure, one can examine `ours` and `theirs` to potentially find a custom solution."] # [doc = " Note that the descriptions of resolutions or resolution failures may be swapped compared"] # [doc = " to the actual changes. This is due to changes like `modification|deletion` being treated the"] # [doc = " same as `deletion|modification`, i.e. *ours* is not more privileged than theirs."] # [doc = " To compensate for that, use [`changes_in_resolution()`](Conflict::changes_in_resolution())."] pub resolution : Result < Resolution , ResolutionFailure > , # [doc = " The change representing *our* side."] pub ours : Change , # [doc = " The change representing *their* side."] pub theirs : Change , # [doc = " An array to store an entry for each stage of the conflict."] # [doc = ""] # [doc = " * `entries[0]`  => Base"] # [doc = " * `entries[1]`  => Ours"] # [doc = " * `entries[2]`  => Theirs"] # [doc = ""] # [doc = " Note that ours and theirs might be swapped, so one should access it through [`Self::entries()`] to compensate for that."] pub entries : [Option < ConflictIndexEntry > ; 3] , # [doc = " Determine how to interpret the `ours` and `theirs` fields. This is used to implement [`Self::changes_in_resolution()`]"] # [doc = " and [`Self::into_parts_by_resolution()`]."] map : ConflictMapping , }
    };
}

Conflict!();