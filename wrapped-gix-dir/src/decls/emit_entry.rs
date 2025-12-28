macro_rules! deps {
    () => {
        Outcome!();
        Action!();
        Delegate!();
        EntryRef!();
        Options!();
        Property!();
        Status!();
        PathspecMatch!();
    };
}

macro_rules! emit_entry {
    () => {
        deps!();
        # [doc = " Possibly emit an entry to `for_each` in case the provided information makes that possible."] # [allow (clippy :: too_many_arguments)] pub (super) fn emit_entry (rela_path : Cow < '_ , BStr > , info : classify :: Outcome , dir_status : Option < entry :: Status > , Options { emit_pruned , emit_tracked , emit_ignored , emit_empty_directories , .. } : Options < '_ > , out : & mut Outcome , delegate : & mut dyn Delegate ,) -> Action { out . seen_entries += 1 ; if (! emit_empty_directories && info . property == Some (entry :: Property :: EmptyDirectory) || ! emit_tracked && info . status == entry :: Status :: Tracked) || emit_ignored . is_none () && matches ! (info . status , entry :: Status :: Ignored (_)) || ! emit_pruned && (info . status . is_pruned () || info . pathspec_match . is_none_or (| m | m == entry :: PathspecMatch :: Excluded)) { return Action :: Continue ; } out . returned_entries += 1 ; delegate . emit (EntryRef :: from_outcome (rela_path , info) , dir_status) }
    };
}

emit_entry!();