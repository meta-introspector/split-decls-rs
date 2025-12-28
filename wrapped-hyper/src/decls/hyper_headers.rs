macro_rules! deps {
    () => {
        OriginalHeaderOrder!();
        HeaderCaseMap!();
    };
}

macro_rules! hyper_headers {
    () => {
        deps!();
        # [doc = " An HTTP header map."] # [doc = ""] # [doc = " These can be part of a request or response."] # [doc = ""] # [doc = " Obtain a pointer to read or modify these from `hyper_request_headers`"] # [doc = " or `hyper_response_headers`."] # [doc = ""] # [doc = " Methods:"] # [doc = ""] # [doc = " - hyper_headers_add:     Adds the provided value to the list of the provided name."] # [doc = " - hyper_headers_foreach: Iterates the headers passing each name and value pair to the callback."] # [doc = " - hyper_headers_set:     Sets the header with the provided name to the provided value."] # [derive (Clone)] pub struct hyper_headers { pub (super) headers : HeaderMap , orig_casing : HeaderCaseMap , orig_order : OriginalHeaderOrder , }
    };
}

hyper_headers!()