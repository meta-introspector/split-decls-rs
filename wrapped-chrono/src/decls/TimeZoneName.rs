macro_rules! TimeZoneName {
    () => {
        # [doc = " ASCII-encoded fixed-capacity string, used for storing time zone names"] # [derive (Copy , Clone , Eq , PartialEq)] struct TimeZoneName { # [doc = " Length-prefixed string buffer"] bytes : [u8 ; 8] , }
    };
}

TimeZoneName!();