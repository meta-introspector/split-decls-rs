macro_rules! deps {
    () => {
        GetResponse!();
        Error!();
        PostResponse!();
        PostBodyDataKind!();
    };
}

macro_rules! Http {
    () => {
        deps!();
        # [doc = " A trait to abstract the HTTP operations needed to power all git interactions: read via GET and write via POST."] # [doc = " Note that 401 must be turned into `std::io::Error(PermissionDenied)`, and other non-success http statuses must be transformed"] # [doc = " into `std::io::Error(Other)`"] # [allow (clippy :: type_complexity)] pub trait Http { # [doc = " A type providing headers line by line."] type Headers : std :: io :: BufRead + Unpin ; # [doc = " A type providing the response."] type ResponseBody : std :: io :: BufRead ; # [doc = " A type allowing to write the content to post."] type PostBody : std :: io :: Write ; # [doc = " Initiate a `GET` request to `url` provided the given `headers`, where `base_url` is so that `base_url + tail == url`."] # [doc = ""] # [doc = " The `base_url` helps to validate redirects and to swap it with the effective base after a redirect."] # [doc = ""] # [doc = " The `headers` are provided verbatim and include both the key as well as the value."] fn get (& mut self , url : & str , base_url : & str , headers : impl IntoIterator < Item = impl AsRef < str > > ,) -> Result < GetResponse < Self :: Headers , Self :: ResponseBody > , Error > ; # [doc = " Initiate a `POST` request to `url` providing with the given `headers`, where `base_url` is so that `base_url + tail == url`."] # [doc = ""] # [doc = " The `base_url` helps to validate redirects and to swap it with the effective base after a redirect."] # [doc = ""] # [doc = " The `headers` are provided verbatim and include both the key as well as the value."] # [doc = " Note that the [`PostResponse`] contains the [`post_body`][PostResponse::post_body] field which implements [`std::io::Write`]"] # [doc = " and is expected to receive the body to post to the server. **It must be dropped** before reading the response"] # [doc = " to prevent deadlocks."] fn post (& mut self , url : & str , base_url : & str , headers : impl IntoIterator < Item = impl AsRef < str > > , body : PostBodyDataKind ,) -> Result < PostResponse < Self :: Headers , Self :: ResponseBody , Self :: PostBody > , Error > ; # [doc = " Pass `config` which can deserialize in the implementation's configuration, as documented separately."] # [doc = ""] # [doc = " The caller must know how that `config` data looks like for the intended implementation."] fn configure (& mut self , config : & dyn std :: any :: Any ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > ; }
    };
}

Http!();