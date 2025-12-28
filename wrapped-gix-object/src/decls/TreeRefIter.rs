macro_rules! TreeRefIter {
    () => {
        # [doc = " A directory snapshot containing files (blobs), directories (trees) and submodules (commits), lazily evaluated."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub struct TreeRefIter < 'a > { # [doc = " The directories and files contained in this tree."] data : & 'a [u8] , }
    };
}

TreeRefIter!()