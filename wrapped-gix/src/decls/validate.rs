macro_rules! deps {
    () => {
        Default!();
        Url!();
        Error!();
        Name!();
        Executable!();
        Fetch!();
        Core!();
        Validate!();
        Boolean!();
        Push!();
        Program!();
        DurationInMilliseconds!();
        Any!();
        LockTimeout!();
        String!();
        PushRefSpec!();
        UnsignedInteger!();
        RemoteName!();
        FetchRefSpec!();
        Path!();
        Http!();
        Time!();
    };
}

macro_rules! validate {
    () => {
        deps!();
        # [doc = " various implementations of the `Validate` trait."] pub mod validate { use std :: { borrow :: Cow , error :: Error } ; use crate :: { bstr :: { BStr , ByteSlice } , config :: tree :: keys :: Validate , remote , } ; # [doc = " Everything is valid."] # [derive (Default)] pub struct All ; impl Validate for All { fn validate (& self , _value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { Ok (()) } } # [doc = " Assure that values that parse as git dates are valid."] # [derive (Default)] pub struct Time ; impl Validate for Time { fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { gix_date :: parse (value . to_str () ? , std :: time :: SystemTime :: now () . into ()) ? ; Ok (()) } } # [doc = " Assure that values that parse as unsigned integers are valid."] # [derive (Default)] pub struct UnsignedInteger ; impl Validate for UnsignedInteger { fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { usize :: try_from (gix_config :: Integer :: try_from (value) ? . to_decimal () . ok_or_else (| | format ! ("integer {value} cannot be represented as `usize`")) ? ,) . map_err (| _ | "cannot use sign for unsigned integer") ? ; Ok (()) } } # [doc = " Assure that values that parse as git booleans are valid."] # [derive (Default)] pub struct Boolean ; impl Validate for Boolean { fn validate (& self , value : & BStr) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { gix_config :: Boolean :: try_from (value) ? ; Ok (()) } } # [doc = " Values that are git remotes, symbolic or urls"] # [derive (Default)] pub struct RemoteName ; impl Validate for RemoteName { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { remote :: Name :: try_from (Cow :: Borrowed (value)) . map_err (| _ | format ! ("Illformed UTF-8 in remote name: \"{}\"" , value . to_str_lossy ())) ? ; Ok (()) } } # [doc = " Values that are programs - everything is allowed."] # [derive (Default)] pub struct Program ; impl Validate for Program { fn validate (& self , _value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { Ok (()) } } # [doc = " Values that are programs executables, everything is allowed."] # [derive (Default)] pub struct Executable ; impl Validate for Executable { fn validate (& self , _value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { Ok (()) } } # [doc = " Values that parse as URLs."] # [derive (Default)] pub struct Url ; impl Validate for Url { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { gix_url :: parse (value) ? ; Ok (()) } } # [doc = " Values that parse as ref-specs for pushing."] # [derive (Default)] pub struct PushRefSpec ; impl Validate for PushRefSpec { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { gix_refspec :: parse (value , gix_refspec :: parse :: Operation :: Push) ? ; Ok (()) } } # [doc = " Values that parse as ref-specs for pushing."] # [derive (Default)] pub struct FetchRefSpec ; impl Validate for FetchRefSpec { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { gix_refspec :: parse (value , gix_refspec :: parse :: Operation :: Fetch) ? ; Ok (()) } } # [doc = " Timeouts used for file locks."] pub struct LockTimeout ; impl Validate for LockTimeout { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { let value = gix_config :: Integer :: try_from (value) ? . to_decimal () . ok_or_else (| | format ! ("integer {value} cannot be represented as integer")) ; super :: super :: Core :: FILES_REF_LOCK_TIMEOUT . try_into_lock_timeout (Ok (value ?)) ? ; Ok (()) } } # [doc = " Durations in milliseconds."] pub struct DurationInMilliseconds ; impl Validate for DurationInMilliseconds { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { let value = gix_config :: Integer :: try_from (value) ? . to_decimal () . ok_or_else (| | format ! ("integer {value} cannot be represented as integer")) ; super :: super :: gitoxide :: Http :: CONNECT_TIMEOUT . try_into_duration (Ok (value ?)) ? ; Ok (()) } } # [doc = " A UTF-8 string."] pub struct String ; impl Validate for String { fn validate (& self , value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { value . to_str () ? ; Ok (()) } } # [doc = " Any path - everything is allowed."] pub struct Path ; impl Validate for Path { fn validate (& self , _value : & BStr) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { Ok (()) } } }
    };
}

validate!();