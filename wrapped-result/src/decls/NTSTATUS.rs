macro_rules! NTSTATUS {
    () => {
        # [doc = " An error or status code value returned by some operating system functions."] # [repr (transparent)] # [derive (Copy , Clone , Default , Eq , PartialEq , Ord , PartialOrd , Hash)] # [must_use] pub struct NTSTATUS (pub i32) ;
    };
}

NTSTATUS!();