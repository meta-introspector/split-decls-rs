macro_rules! AsyncStatus {
    () => {
        # [repr (transparent)] # [derive (Clone , Copy , Debug , Default , Eq , PartialEq)] pub struct AsyncStatus (pub i32) ;
    };
}

AsyncStatus!()