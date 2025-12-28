macro_rules! deps {
    () => {
        BatchResponse!();
        CacheControl!();
        Response!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl BatchResponse { # [doc = " Gets cache control value"] pub fn cache_control (& self) -> CacheControl { match self { BatchResponse :: Single (resp) => resp . cache_control , BatchResponse :: Batch (resp) => resp . iter () . fold (CacheControl :: default () , | acc , item | { acc . merge (& item . cache_control) }) , } } # [doc = " Returns `true` if all responses are ok."] pub fn is_ok (& self) -> bool { match self { BatchResponse :: Single (resp) => resp . is_ok () , BatchResponse :: Batch (resp) => resp . iter () . all (Response :: is_ok) , } } # [doc = " Returns HTTP headers map."] pub fn http_headers (& self) -> http :: HeaderMap { match self { BatchResponse :: Single (resp) => resp . http_headers . clone () , BatchResponse :: Batch (resp) => { resp . iter () . fold (http :: HeaderMap :: new () , | mut acc , resp | { acc . extend (resp . http_headers . clone ()) ; acc }) } } } # [doc = " Returns HTTP headers iterator."] pub fn http_headers_iter (& self) -> impl Iterator < Item = (http :: HeaderName , http :: HeaderValue) > { let headers = self . http_headers () ; let mut current_name = None ; headers . into_iter () . filter_map (move | (name , value) | { if let Some (name) = name { current_name = Some (name) ; } current_name . clone () . map (| current_name | (current_name , value)) }) } }
    };
}

impl_112!()