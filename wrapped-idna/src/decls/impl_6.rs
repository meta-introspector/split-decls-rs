macro_rules! deps {
    () => {
        ErrorPolicy!();
        Config!();
        ProcessingError!();
        Idna!();
        ProcessingSuccess!();
        Uts46!();
        Errors!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Idna { pub fn new (config : Config) -> Self { Self { config } } # [doc = " [UTS 46 ToASCII](http://www.unicode.org/reports/tr46/#ToASCII)"] # [allow (clippy :: wrong_self_convention)] pub fn to_ascii (& mut self , domain : & str , out : & mut String) -> Result < () , Errors > { let mapped = map_transitional (domain , self . config . transitional_processing) ; match Uts46 :: new () . process (mapped . as_bytes () , self . config . deny_list () , self . config . hyphens () , ErrorPolicy :: FailFast , | _ , _ , _ | false , out , None ,) { Ok (ProcessingSuccess :: Passthrough) => { if self . config . verify_dns_length && ! verify_dns_length (& mapped , true) { return Err (crate :: Errors :: default ()) ; } out . push_str (& mapped) ; Ok (()) } Ok (ProcessingSuccess :: WroteToSink) => { if self . config . verify_dns_length && ! verify_dns_length (out , true) { return Err (crate :: Errors :: default ()) ; } Ok (()) } Err (ProcessingError :: ValidityError) => Err (crate :: Errors :: default ()) , Err (ProcessingError :: SinkError) => unreachable ! () , } } # [doc = " [UTS 46 ToUnicode](http://www.unicode.org/reports/tr46/#ToUnicode)"] # [allow (clippy :: wrong_self_convention)] pub fn to_unicode (& mut self , domain : & str , out : & mut String) -> Result < () , Errors > { let mapped = map_transitional (domain , self . config . transitional_processing) ; match Uts46 :: new () . process (mapped . as_bytes () , self . config . deny_list () , self . config . hyphens () , ErrorPolicy :: MarkErrors , | _ , _ , _ | true , out , None ,) { Ok (ProcessingSuccess :: Passthrough) => { out . push_str (& mapped) ; Ok (()) } Ok (ProcessingSuccess :: WroteToSink) => Ok (()) , Err (ProcessingError :: ValidityError) => Err (crate :: Errors :: default ()) , Err (ProcessingError :: SinkError) => unreachable ! () , } } }
    };
}

impl_6!()