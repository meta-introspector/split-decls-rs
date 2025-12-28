macro_rules! NonZero {
    () => {
        # [doc = " Wrapper type for non-zero integers."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] # [repr (transparent)] pub struct NonZero < T : ? Sized > (pub (crate) T) ;
    };
}

NonZero!();