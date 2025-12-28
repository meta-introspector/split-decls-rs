macro_rules! deps {
    () => {
        Error!();
        Boolean!();
        Key!();
    };
}

macro_rules! config_bool {
    () => {
        deps!();
        pub (crate) fn config_bool (config : & gix_config :: File < '_ > , key : & 'static config :: tree :: keys :: Boolean , key_str : & str , default : bool , lenient : bool ,) -> Result < bool , Error > { use config :: tree :: Key ; debug_assert_eq ! (key_str , key . logical_name () , "BUG: key name and hardcoded name must match") ; config . boolean (key_str) . map_or (Ok (default) , | res | key . enrich_error (res)) . map_err (Error :: from) . with_lenient_default (lenient) }
    };
}

config_bool!();