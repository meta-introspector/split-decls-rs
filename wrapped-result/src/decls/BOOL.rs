macro_rules! BOOL {
    () => {
        # [doc = " A 32-bit value representing boolean values and returned by some functions to indicate success or failure."] # [must_use] # [repr (transparent)] # [derive (Clone , Copy , Debug , PartialEq , Eq , Default)] pub struct BOOL (pub i32) ;
    };
}

BOOL!();