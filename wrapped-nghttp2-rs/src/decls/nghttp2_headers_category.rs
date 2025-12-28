macro_rules! nghttp2_headers_category {
    () => {
        # [doc = " @enum"] # [doc = ""] # [doc = " The category of HEADERS, which indicates the role of the frame.  In"] # [doc = " HTTP/2 spec, request, response, push response and other arbitrary"] # [doc = " headers (e.g., trailer fields) are all called just HEADERS.  To"] # [doc = " give the application the role of incoming HEADERS frame, we define"] # [doc = " several categories."] pub type nghttp2_headers_category = u32 ;
    };
}

nghttp2_headers_category!()