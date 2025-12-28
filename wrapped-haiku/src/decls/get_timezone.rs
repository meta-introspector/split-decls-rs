macro_rules! get_timezone {
    () => {
        # [doc = " Get the current IANA time zone as a string."] # [doc = ""] # [doc = " On Haiku platforms this function will return [`Some`] with the timezone string"] # [doc = " or [`None`] if an error occurs. On all other platforms, [`None`] is returned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let timezone = iana_time_zone_haiku::get_timezone();"] # [doc = " ```"] # [must_use] pub fn get_timezone () -> Option < String > { let mut buf = [0u8 ; 64] ; let len = unsafe { let buf_size = buf . len () ; let buf_ptr = buf . as_mut_ptr () . cast :: < c_char > () ; iana_time_zone_haiku_get_tz (buf_ptr , buf_size) } ; match buf . get (.. len) ? { b"" => None , s => Some (std :: str :: from_utf8 (s) . ok () ? . to_owned ()) , } }
    };
}

get_timezone!();