macro_rules! deps {
    () => {
        ConflictMapping!();
        Error!();
    };
}

macro_rules! apply_our_resolution {
    () => {
        deps!();
        fn apply_our_resolution (local_ours : & Change , local_theirs : & Change , outer_side : ConflictMapping , editor : & mut gix_object :: tree :: Editor < '_ > ,) -> Result < () , Error > { let ours = match outer_side { Original => local_ours , Swapped => local_theirs , } ; Ok (apply_change (editor , ours , None) ?) }
    };
}

apply_our_resolution!();