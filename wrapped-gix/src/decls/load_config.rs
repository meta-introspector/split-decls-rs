macro_rules! deps {
    () => {
        Options!();
        Error!();
    };
}

macro_rules! load_config {
    () => {
        deps!();
        fn load_config (config_path : std :: path :: PathBuf , buf : & mut Vec < u8 > , source : gix_config :: Source , git_dir_trust : gix_sec :: Trust , lossy : bool , lenient : bool ,) -> Result < gix_config :: File < 'static > , Error > { let metadata = gix_config :: file :: Metadata :: from (source) . at (& config_path) . with (git_dir_trust) ; let mut file = match std :: fs :: File :: open (& config_path) { Ok (f) => f , Err (err) if err . kind () == std :: io :: ErrorKind :: NotFound => return Ok (gix_config :: File :: new (metadata)) , Err (err) => { let err = Error :: Io { source : err , path : config_path , } ; if lenient { gix_trace :: warn ! ("ignoring: {err:#?}") ; return Ok (gix_config :: File :: new (metadata)) ; } else { return Err (err) ; } } } ; buf . clear () ; if let Err (err) = std :: io :: copy (& mut file , buf) { let err = Error :: Io { source : err , path : config_path , } ; if lenient { gix_trace :: warn ! ("ignoring: {err:#?}") ; buf . clear () ; } else { return Err (err) ; } } let config = gix_config :: File :: from_bytes_owned (buf , metadata , gix_config :: file :: init :: Options { includes : gix_config :: file :: includes :: Options :: no_follow () , .. util :: base_options (lossy , lenient) } ,) ? ; Ok (config) }
    };
}

load_config!()