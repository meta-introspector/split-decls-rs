macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! compress_data {
    () => {
        deps!();
        fn compress_data (obj : & gix_object :: Data < '_ >) -> Result < Vec < u8 > , input :: Error > { let mut out = gix_features :: zlib :: stream :: deflate :: Write :: new (Vec :: new ()) ; if let Err (err) = std :: io :: copy (& mut & * obj . data , & mut out) { match err . kind () { std :: io :: ErrorKind :: Other => return Err (input :: Error :: Io (err . into ())) , err => { unreachable ! ("Should never see other errors than zlib, but got {:?}" , err) } } } out . flush () . expect ("zlib flush should never fail") ; Ok (out . into_inner ()) }
    };
}

compress_data!();