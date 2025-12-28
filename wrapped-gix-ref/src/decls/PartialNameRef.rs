macro_rules! PartialNameRef {
    () => {
        # [doc = " A validated and potentially partial reference name, safe to use for common operations."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] # [repr (transparent)] pub struct PartialNameRef (BStr) ;
    };
}

PartialNameRef!()