macro_rules! deps {
    () => {
        EntryRef!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " Just like [`EntryRef`], but with all fields owned (and thus without a lifetime to consider)."] # [derive (Debug , Clone , PartialEq , Eq , Hash , Ord , PartialOrd)] pub struct Entry { # [doc = " See [EntryRef::rela_path] for details."] pub rela_path : BString , # [doc = " The status of entry, most closely related to what we know from `git status`, but not the same."] pub status : entry :: Status , # [doc = " Additional flags that further clarify properties of the entry."] pub property : Option < entry :: Property > , # [doc = " Further specify what the entry is on disk, similar to a file mode."] pub disk_kind : Option < entry :: Kind > , # [doc = " The kind of entry according to the index, if tracked. *Usually* the same as `disk_kind`."] # [doc = " Note that even if tracked, this might be `None` which indicates this is a worktree placed"] # [doc = " within the parent repository."] pub index_kind : Option < entry :: Kind > , # [doc = " Indicate how the pathspec matches the entry. See more in [`EntryRef::pathspec_match`]."] pub pathspec_match : Option < entry :: PathspecMatch > , }
    };
}

Entry!()