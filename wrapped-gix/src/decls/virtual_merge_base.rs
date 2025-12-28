macro_rules! deps {
    () => {
        Outcome!();
        Id!();
    };
}

macro_rules! virtual_merge_base {
    () => {
        deps!();
        # [doc = ""] pub mod virtual_merge_base { use crate :: Id ; # [doc = " The outcome produced by [`Repository::virtual_merge_base()`](crate::Repository::virtual_merge_base())."] pub struct Outcome < 'repo > { # [doc = " The commit ids of all the virtual merge bases we have produced in the process of recursively merging the merge-bases."] # [doc = " As they have been written to the object database, they are still available until they are garbage collected."] # [doc = " The last one is the most recently produced and the one returned as `commit_id`."] # [doc = " If this list is empty, this means that there was only one merge-base, which itself is already suitable the final merge-base."] pub virtual_merge_bases : Vec < Id < 'repo > > , # [doc = " The id of the commit that was created to hold the merged tree."] pub commit_id : Id < 'repo > , # [doc = " The hash of the merged tree."] pub tree_id : Id < 'repo > , } }
    };
}

virtual_merge_base!();