macro_rules! HRESULT {
    () => {
        # [doc = " An error code value returned by most COM functions."] # [repr (transparent)] # [derive (Copy , Clone , Default , Eq , PartialEq , Ord , PartialOrd , Hash)] # [must_use] pub struct HRESULT (pub i32) ;
    };
}

HRESULT!()