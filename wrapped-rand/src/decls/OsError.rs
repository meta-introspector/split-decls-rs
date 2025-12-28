macro_rules! deps {
    () => {
        Error!();
        OsRng!();
    };
}

macro_rules! OsError {
    () => {
        deps!();
        # [doc = " Error type of [`OsRng`]"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct OsError (getrandom :: Error) ;
    };
}

OsError!();