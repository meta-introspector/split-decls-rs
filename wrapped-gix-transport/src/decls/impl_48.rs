macro_rules! deps {
    () => {
        Handler!();
        Error!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Handler { fn reset (& mut self) { self . checked_status = false ; self . last_status = 0 ; self . follow = FollowRedirects :: default () ; } fn parse_status_inner (data : & [u8]) -> Result < usize , Box < dyn std :: error :: Error + Send + Sync > > { let code = data . split (| b | * b == b' ') . nth (1) . ok_or ("Expected HTTP/<VERSION> STATUS") ? ; let code = std :: str :: from_utf8 (code) ? ; code . parse () . map_err (Into :: into) } fn parse_status (data : & [u8] , follow : FollowRedirects) -> Option < (usize , Box < dyn std :: error :: Error + Send + Sync >) > { let valid_end = match follow { FollowRedirects :: Initial | FollowRedirects :: All => 308 , FollowRedirects :: None => 299 , } ; match Self :: parse_status_inner (data) { Ok (status) if ! (200 ..= valid_end) . contains (& status) => { Some ((status , format ! ("Received HTTP status {status}") . into ())) } Ok (_) => None , Err (err) => Some ((500 , err)) , } } }
    };
}

impl_48!()