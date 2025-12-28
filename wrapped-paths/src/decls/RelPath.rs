macro_rules! RelPath {
    () => {
        # [doc = " Wrapper around a relative [`Utf8Path`]."] # [derive (Debug , Ord , PartialOrd , Eq , PartialEq , Hash)] # [repr (transparent)] pub struct RelPath (Utf8Path) ;
    };
}

RelPath!();