macro_rules! deps {
    () => {
        Curl!();
        GetResponse!();
        PostBodyDataKind!();
        Error!();
        PostResponse!();
        Http!();
        Options!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        # [allow (clippy :: type_complexity)] impl http :: Http for Curl { type Headers = io :: pipe :: Reader ; type ResponseBody = io :: pipe :: Reader ; type PostBody = io :: pipe :: Writer ; fn get (& mut self , url : & str , base_url : & str , headers : impl IntoIterator < Item = impl AsRef < str > > ,) -> Result < http :: GetResponse < Self :: Headers , Self :: ResponseBody > , http :: Error > { self . make_request (url , base_url , headers , None) . map (Into :: into) } fn post (& mut self , url : & str , base_url : & str , headers : impl IntoIterator < Item = impl AsRef < str > > , body : PostBodyDataKind ,) -> Result < http :: PostResponse < Self :: Headers , Self :: ResponseBody , Self :: PostBody > , http :: Error > { self . make_request (url , base_url , headers , Some (body)) } fn configure (& mut self , config : & dyn std :: any :: Any ,) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { if let Some (config) = config . downcast_ref :: < http :: Options > () { self . config = config . clone () ; } Ok (()) } }
    };
}

impl_64!()