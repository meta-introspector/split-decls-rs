macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use std :: path :: { Path , PathBuf } ; # [test] fn entry_by_path_with_conflicting_file () { let file = PathBuf :: from ("tests") . join ("fixtures") . join (Path :: new ("loose_index") . join ("conflicting-file.git-index")) ; let file = crate :: File :: at (file , gix_hash :: Kind :: Sha1 , false , Default :: default ()) . expect ("valid file") ; assert_eq ! (file . entries () . len () , 3 , "we have a set of conflict entries for a single file") ; for idx in 0 .. 3 { for wanted_stage in 1 ..= 3 { let actual_idx = file . entry_index_by_idx_and_stage ("file" . into () , idx , wanted_stage , (idx + 1) . cmp (& (wanted_stage as usize)) ,) . expect ("found") ; assert_eq ! (actual_idx + 1 , wanted_stage as usize , "the index and stage have a relation, and that is upheld if we search correctly") ; } } } }
    };
}

tests!();