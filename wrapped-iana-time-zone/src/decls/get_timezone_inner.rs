macro_rules! deps {
    () => {
        GetTimezoneError!();
    };
}

macro_rules! get_timezone_inner {
    () => {
        deps!();
        pub fn get_timezone_inner () -> Result < String , crate :: GetTimezoneError > { Err (crate :: GetTimezoneError :: OsError) }
    };
}

get_timezone_inner!();