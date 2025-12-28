macro_rules! deps {
    () => {
        Response!();
        ServerError!();
        Result!();
        CacheControl!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl Response { # [doc = " Create a new successful response with the data."] # [must_use] pub fn new (data : impl Into < Value >) -> Self { Self { data : data . into () , .. Default :: default () } } # [doc = " Create a response from some errors."] # [must_use] pub fn from_errors (errors : Vec < ServerError >) -> Self { Self { errors , .. Default :: default () } } # [doc = " Set the extension result of the response."] # [must_use] pub fn extension (mut self , name : impl Into < String > , value : Value) -> Self { self . extensions . insert (name . into () , value) ; self } # [doc = " Set the http headers of the response."] # [must_use] pub fn http_headers (self , http_headers : http :: HeaderMap) -> Self { Self { http_headers , .. self } } # [doc = " Set the cache control of the response."] # [must_use] pub fn cache_control (self , cache_control : CacheControl) -> Self { Self { cache_control , .. self } } # [doc = " Returns `true` if the response is ok."] # [inline] pub fn is_ok (& self) -> bool { self . errors . is_empty () } # [doc = " Returns `true` if the response is error."] # [inline] pub fn is_err (& self) -> bool { ! self . is_ok () } # [doc = " Extract the error from the response. Only if the `error` field is empty"] # [doc = " will this return `Ok`."] # [inline] pub fn into_result (self) -> Result < Self , Vec < ServerError > > { if self . is_err () { Err (self . errors) } else { Ok (self) } } }
    };
}

impl_110!()