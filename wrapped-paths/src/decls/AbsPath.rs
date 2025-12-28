macro_rules! AbsPath {
    () => {
        # [doc = " Wrapper around an absolute [`Utf8Path`]."] # [derive (Debug , Ord , PartialOrd , Eq , Hash)] # [repr (transparent)] pub struct AbsPath (Utf8Path) ;
    };
}

AbsPath!();