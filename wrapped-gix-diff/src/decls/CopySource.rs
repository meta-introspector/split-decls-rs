macro_rules! CopySource {
    () => {
        # [doc = " Determine in which set of files to search for copies."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq)] pub enum CopySource { # [doc = " Find copies from the set of modified files only."] # [default] FromSetOfModifiedFiles , # [doc = " Find copies from the set of modified files, as well as all files known to the source (i.e. previous state of the tree)."] # [doc = ""] # [doc = " This can be an expensive operation as it scales exponentially with the total amount of files in the set."] FromSetOfModifiedFilesAndAllSources , }
    };
}

CopySource!();