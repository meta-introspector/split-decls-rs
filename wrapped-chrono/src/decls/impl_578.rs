macro_rules! deps {
    () => {
        Fixed!();
        LocalTimeType!();
        TimeZoneRef!();
        AlternateTime!();
        TransitionRule!();
        MappedLocalTime!();
        NaiveDateTime!();
        TimeZone!();
        LeapSecond!();
        Error!();
        Transition!();
    };
}

macro_rules! impl_578 {
    () => {
        deps!();
        impl TimeZone { # [doc = " Returns local time zone."] # [doc = ""] # [doc = " This method in not supported on non-UNIX platforms, and returns the UTC time zone instead."] pub (crate) fn local (env_tz : Option < & str >) -> Result < Self , Error > { match env_tz { Some (tz) => Self :: from_posix_tz (tz) , None => Self :: from_posix_tz ("localtime") , } } # [doc = " Construct a time zone from a POSIX TZ string, as described in [the POSIX documentation of the `TZ` environment variable](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap08.html)."] fn from_posix_tz (tz_string : & str) -> Result < Self , Error > { if tz_string . is_empty () { return Ok (Self :: utc ()) ; } if tz_string == "localtime" { return Self :: from_tz_data (& fs :: read ("/etc/localtime") ?) ; } # [cfg (any (target_os = "android" , target_env = "ohos"))] { if let Ok (Some (bytes)) = crate :: offset :: local :: tz_data :: for_zone (tz_string) { return Self :: from_tz_data (& bytes) ; } } let mut chars = tz_string . chars () ; if chars . next () == Some (':') { return Self :: from_file (& mut find_tz_file (chars . as_str ()) ?) ; } if let Ok (mut file) = find_tz_file (tz_string) { return Self :: from_file (& mut file) ; } let tz_string = tz_string . trim_matches (| c : char | c . is_ascii_whitespace ()) ; let rule = TransitionRule :: from_tz_string (tz_string . as_bytes () , false) ? ; Self :: new (vec ! [] , match rule { TransitionRule :: Fixed (local_time_type) => vec ! [local_time_type] , TransitionRule :: Alternate (AlternateTime { std , dst , .. }) => vec ! [std , dst] , } , vec ! [] , Some (rule) ,) } # [doc = " Construct a time zone"] pub (super) fn new (transitions : Vec < Transition > , local_time_types : Vec < LocalTimeType > , leap_seconds : Vec < LeapSecond > , extra_rule : Option < TransitionRule > ,) -> Result < Self , Error > { let new = Self { transitions , local_time_types , leap_seconds , extra_rule } ; new . as_ref () . validate () ? ; Ok (new) } # [doc = " Construct a time zone from the contents of a time zone file"] fn from_file (file : & mut File) -> Result < Self , Error > { let mut bytes = Vec :: new () ; file . read_to_end (& mut bytes) ? ; Self :: from_tz_data (& bytes) } # [doc = " Construct a time zone from the contents of a time zone file"] # [doc = ""] # [doc = " Parse TZif data as described in [RFC 8536](https://datatracker.ietf.org/doc/html/rfc8536)."] pub (crate) fn from_tz_data (bytes : & [u8]) -> Result < Self , Error > { parser :: parse (bytes) } # [doc = " Construct a time zone with the specified UTC offset in seconds"] fn fixed (ut_offset : i32) -> Result < Self , Error > { Ok (Self { transitions : Vec :: new () , local_time_types : vec ! [LocalTimeType :: with_offset (ut_offset) ?] , leap_seconds : Vec :: new () , extra_rule : None , }) } # [doc = " Construct the time zone associated to UTC"] pub (crate) fn utc () -> Self { Self { transitions : Vec :: new () , local_time_types : vec ! [LocalTimeType :: UTC] , leap_seconds : Vec :: new () , extra_rule : None , } } # [doc = " Find the local time type associated to the time zone at the specified Unix time in seconds"] pub (crate) fn find_local_time_type (& self , unix_time : i64) -> Result < & LocalTimeType , Error > { self . as_ref () . find_local_time_type (unix_time) } pub (crate) fn find_local_time_type_from_local (& self , local_time : NaiveDateTime ,) -> Result < crate :: MappedLocalTime < LocalTimeType > , Error > { self . as_ref () . find_local_time_type_from_local (local_time) } # [doc = " Returns a reference to the time zone"] fn as_ref (& self) -> TimeZoneRef < '_ > { TimeZoneRef { transitions : & self . transitions , local_time_types : & self . local_time_types , leap_seconds : & self . leap_seconds , extra_rule : & self . extra_rule , } } }
    };
}

impl_578!();