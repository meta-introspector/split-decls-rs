macro_rules! Size {
    () => {
        # [doc = " A strongly typed a size."] # [derive (PartialEq , Eq , Hash , PartialOrd , Ord , Debug , Clone , Copy)] pub (crate) struct Size (pub (crate) usize) ;
    };
}

Size!()