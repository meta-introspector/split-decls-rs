macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! ResourceRef {
    () => {
        deps!();
        # [doc = " A blob or executable ready to be merged in one way or another."] # [derive (Copy , Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] pub struct ResourceRef < 'a > { # [doc = " The data itself, suitable for merging, and if the object or worktree item is present at all."] pub data : resource :: Data < 'a > , # [doc = " The location of the resource, relative to the working tree."] pub rela_path : & 'a BStr , # [doc = " The id of the content as it would be stored in `git`, or `null` if the content doesn't exist anymore at"] # [doc = " `rela_path` or if it was never computed. This can happen with content read from the worktree, which"] # [doc = " after its 'to-git' conversion never had its hash computed."] pub id : & 'a gix_hash :: oid , }
    };
}

ResourceRef!()