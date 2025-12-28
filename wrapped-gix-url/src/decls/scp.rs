macro_rules! deps {
    () => {
        ParsedUrl!();
        Url!();
        Error!();
        UrlKind!();
        Scheme!();
    };
}

macro_rules! scp {
    () => {
        deps!();
        pub (crate) fn scp (input : & BStr , colon : usize) -> Result < crate :: Url , Error > { let input = input_to_utf8 (input , UrlKind :: Scp) ? ; let (host , path) = input . split_at (colon) ; debug_assert_eq ! (path . get (.. 1) , Some (":") , "{path} should start with :") ; let path = & path [1 ..] ; if path . is_empty () { return Err (Error :: MissingRepositoryPath { url : input . to_owned () . into () , kind : UrlKind :: Scp , }) ; } let url_string = format ! ("ssh://{host}") ; let url = crate :: simple_url :: ParsedUrl :: parse (& url_string) . map_err (| source | Error :: Url { url : input . to_owned () , kind : UrlKind :: Scp , source , }) ? ; Ok (crate :: Url { serialize_alternative_form : true , scheme : Scheme :: from (url . scheme . as_str ()) , user : url_user (& url , UrlKind :: Scp) ? , password : url . password . map (| s | percent_decoded_utf8 (s , UrlKind :: Scp)) . transpose () ? , host : url . host , port : url . port , path : path . into () , }) }
    };
}

scp!();