macro_rules! deps {
    () => {
        GetTimezoneError!();
    };
}

macro_rules! get_timezone {
    () => {
        deps!();
        # [doc = " Get the current IANA time zone as a string."] # [doc = ""] # [doc = " See the module-level documentation for a usage example and more details"] # [doc = " about this function."] # [inline] pub fn get_timezone () -> Result < String , GetTimezoneError > { platform :: get_timezone_inner () }
    };
}

get_timezone!()