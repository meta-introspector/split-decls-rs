macro_rules! WIN32_ERROR {
    () => {
        # [doc = " An error or status code value returned by some operating system functions."] # [repr (transparent)] # [derive (Copy , Clone , Default , Eq , PartialEq , Ord , PartialOrd , Hash)] # [must_use] pub struct WIN32_ERROR (pub u32) ;
    };
}

WIN32_ERROR!()