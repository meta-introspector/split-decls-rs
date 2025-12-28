macro_rules! deps {
    () => {
        LocalTimeType!();
        Error!();
        TimeZoneName!();
    };
}

macro_rules! impl_590 {
    () => {
        deps!();
        impl LocalTimeType { # [doc = " Construct a local time type"] pub (super) fn new (ut_offset : i32 , is_dst : bool , name : Option < & [u8] >) -> Result < Self , Error > { if ut_offset == i32 :: MIN { return Err (Error :: LocalTimeType ("invalid UTC offset")) ; } let name = match name { Some (name) => TimeZoneName :: new (name) ? , None => return Ok (Self { ut_offset , is_dst , name : None }) , } ; Ok (Self { ut_offset , is_dst , name : Some (name) }) } # [doc = " Construct a local time type with the specified UTC offset in seconds"] pub (super) const fn with_offset (ut_offset : i32) -> Result < Self , Error > { if ut_offset == i32 :: MIN { return Err (Error :: LocalTimeType ("invalid UTC offset")) ; } Ok (Self { ut_offset , is_dst : false , name : None }) } # [doc = " Returns offset from UTC in seconds"] pub (crate) const fn offset (& self) -> i32 { self . ut_offset } # [doc = " Returns daylight saving time indicator"] pub (super) const fn is_dst (& self) -> bool { self . is_dst } pub (super) const UTC : LocalTimeType = Self { ut_offset : 0 , is_dst : false , name : None } ; }
    };
}

impl_590!();