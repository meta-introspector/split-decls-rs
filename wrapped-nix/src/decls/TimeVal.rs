macro_rules! TimeVal {
    () => {
        # [repr (transparent)] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct TimeVal (timeval) ;
    };
}

TimeVal!()