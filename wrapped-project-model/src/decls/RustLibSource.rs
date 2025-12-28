macro_rules! RustLibSource {
    () => {
        # [doc = " Describes how to set the rustc source directory."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum RustLibSource { # [doc = " Explicit path for the rustc source directory."] Path (AbsPathBuf) , # [doc = " Try to automatically detect where the rustc source directory is."] Discover , }
    };
}

RustLibSource!();