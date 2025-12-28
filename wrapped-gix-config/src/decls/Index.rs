macro_rules! Index {
    () => {
        # [doc = " A strongly typed index into some range."] # [derive (PartialEq , Eq , Hash , PartialOrd , Ord , Debug , Clone , Copy)] pub (crate) struct Index (pub (crate) usize) ;
    };
}

Index!();