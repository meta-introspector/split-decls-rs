macro_rules! deps {
    () => {
        Curl!();
        PostBodyDataKind!();
        PostResponse!();
        Response!();
        Error!();
        Request!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Curl { fn restore_thread_after_failure (& mut self) -> http :: Error { let err_that_brought_thread_down = self . handle . take () . expect ("thread handle present") . join () . expect ("handler thread should never panic") . expect_err ("something should have gone wrong with curl (we join on error only)") ; let (handle , req , res) = remote :: new () ; self . handle = Some (handle) ; self . req = req ; self . res = res ; err_that_brought_thread_down . into () } fn make_request (& mut self , url : & str , base_url : & str , headers : impl IntoIterator < Item = impl AsRef < str > > , upload_body_kind : Option < PostBodyDataKind > ,) -> Result < http :: PostResponse < io :: pipe :: Reader , io :: pipe :: Reader , io :: pipe :: Writer > , http :: Error > { let mut list = curl :: easy :: List :: new () ; for header in headers { list . append (header . as_ref ()) ? ; } if self . req . send (remote :: Request { url : url . to_owned () , base_url : base_url . to_owned () , headers : list , upload_body_kind , config : self . config . clone () , }) . is_err () { return Err (self . restore_thread_after_failure ()) ; } let remote :: Response { headers , body , upload_body , } = match self . res . recv () { Ok (res) => res , Err (_) => return Err (self . restore_thread_after_failure ()) , } ; Ok (http :: PostResponse { post_body : upload_body , headers , body , }) } }
    };
}

impl_62!();