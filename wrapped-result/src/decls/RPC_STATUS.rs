macro_rules! RPC_STATUS {
    () => {
        # [doc = " An error or status code value returned by some operating system functions."] # [repr (transparent)] # [derive (Copy , Clone , Default , Eq , PartialEq , Ord , PartialOrd , Hash)] # [must_use] pub struct RPC_STATUS (pub i32) ;
    };
}

RPC_STATUS!();