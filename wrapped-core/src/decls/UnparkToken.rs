macro_rules! UnparkToken {
    () => {
        # [doc = " A value which is passed from an unparker to a parked thread."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub struct UnparkToken (pub usize) ;
    };
}

UnparkToken!();