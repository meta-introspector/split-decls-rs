macro_rules! Boolean {
    () => {
        # [doc = " Any value that can be interpreted as a boolean."] # [derive (Default , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] # [allow (missing_docs)] pub struct Boolean (pub bool) ;
    };
}

Boolean!();