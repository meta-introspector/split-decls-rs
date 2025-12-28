macro_rules! deps {
    () => {
        Error!();
        Result!();
        Context!();
    };
}

macro_rules! mutate {
    () => {
        deps!();
        mod mutate { use bstr :: ByteSlice ; use crate :: { protocol , protocol :: Context } ; # [doc = " In-place mutation"] impl Context { # [doc = " Destructure the url at our `url` field into parts like protocol, host, username and path and store"] # [doc = " them in our respective fields. If `use_http_path` is set, http paths are significant even though"] # [doc = " normally this isn't the case."] # [allow (clippy :: result_large_err)] pub fn destructure_url_in_place (& mut self , use_http_path : bool) -> Result < & mut Self , protocol :: Error > { if self . url . is_none () { self . url = Some (self . to_url () . ok_or (protocol :: Error :: UrlMissing) ?) ; } let url = gix_url :: parse (self . url . as_ref () . expect ("URL is present after check above") . as_ref ()) ? ; self . protocol = Some (url . scheme . as_str () . into ()) ; self . username = url . user () . map (ToOwned :: to_owned) ; self . password = url . password () . map (ToOwned :: to_owned) ; self . host = url . host () . map (ToOwned :: to_owned) . map (| mut host | { let port = url . port . filter (| port | { url . scheme . default_port () . is_none_or (| default_port | * port != default_port) }) ; if let Some (port) = port { use std :: fmt :: Write ; write ! (host , ":{port}") . expect ("infallible") ; } host }) ; if ! matches ! (url . scheme , gix_url :: Scheme :: Http | gix_url :: Scheme :: Https) || use_http_path { let path = url . path . trim_with (| b | b == '/') ; self . path = (! path . is_empty ()) . then (| | path . into ()) ; } Ok (self) } } }
    };
}

mutate!();