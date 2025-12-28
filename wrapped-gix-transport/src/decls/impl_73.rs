macro_rules! deps {
    () => {
        Options!();
        Error!();
        PostBodyDataKind!();
        Remote!();
        Http!();
        GetResponse!();
        PostResponse!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl http :: Http for Remote { type Headers = pipe :: Reader ; type ResponseBody = pipe :: Reader ; type PostBody = pipe :: Writer ; fn get (& mut self , url : & str , base_url : & str , headers : impl IntoIterator < Item = impl AsRef < str > > ,) -> Result < http :: GetResponse < Self :: Headers , Self :: ResponseBody > , http :: Error > { self . make_request (url , base_url , headers , None) . map (Into :: into) } fn post (& mut self , url : & str , base_url : & str , headers : impl IntoIterator < Item = impl AsRef < str > > , post_body_kind : PostBodyDataKind ,) -> Result < http :: PostResponse < Self :: Headers , Self :: ResponseBody , Self :: PostBody > , http :: Error > { self . make_request (url , base_url , headers , Some (post_body_kind)) } fn configure (& mut self , config : & dyn Any) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { if let Some (config) = config . downcast_ref :: < http :: Options > () { self . config = config . clone () ; } Ok (()) } }
    };
}

impl_73!()