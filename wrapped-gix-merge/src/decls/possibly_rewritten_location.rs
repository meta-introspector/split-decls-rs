macro_rules! deps {
    () => {
        ChangeListRef!();
        TreeNodes!();
        PossibleConflict!();
    };
}

macro_rules! possibly_rewritten_location {
    () => {
        deps!();
        # [doc = " Assuming that `their_location` is the destination of *their* rewrite, check if *it* passes"] # [doc = " over a directory rewrite in *our* tree. If so, rewrite it so that we get the path"] # [doc = " it would have had if it had been renamed along with *our* directory."] pub fn possibly_rewritten_location (check_tree : & TreeNodes , their_location : & BStr , our_changes : & ChangeListRef ,) -> Option < BString > { check_tree . check_conflict (their_location) . and_then (| pc | match pc { PossibleConflict :: PassedRewrittenDirectory { change_idx } => { let passed_change = & our_changes [change_idx] ; rewrite_location_with_renamed_directory (their_location , & passed_change . inner) } _ => None , }) }
    };
}

possibly_rewritten_location!();