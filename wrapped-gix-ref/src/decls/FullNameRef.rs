macro_rules! FullNameRef {
    () => {
        # [doc = " A validated complete and fully qualified reference name, safe to use for all operations."] # [derive (Hash , Debug , PartialEq , Eq , Ord , PartialOrd)] # [repr (transparent)] pub struct FullNameRef (BStr) ;
    };
}

FullNameRef!();