macro_rules! TimeSpec {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct TimeSpec (timespec) ;
    };
}

TimeSpec!()