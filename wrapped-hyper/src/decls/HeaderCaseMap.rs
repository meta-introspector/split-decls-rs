macro_rules! HeaderCaseMap {
    () => {
        # [doc = " A map from header names to their original casing as received in an HTTP message."] # [doc = ""] # [doc = " If an HTTP/1 response `res` is parsed on a connection whose option"] # [doc = " [`preserve_header_case`] was set to true and the response included"] # [doc = " the following headers:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " x-Bread: Baguette"] # [doc = " X-BREAD: Pain"] # [doc = " x-bread: Ficelle"] # [doc = " ```"] # [doc = ""] # [doc = " Then `res.extensions().get::<HeaderCaseMap>()` will return a map with:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " HeaderCaseMap({"] # [doc = "     \"x-bread\": [\"x-Bread\", \"X-BREAD\", \"x-bread\"],"] # [doc = " })"] # [doc = " ```"] # [doc = ""] # [doc = " [`preserve_header_case`]: /client/struct.Client.html#method.preserve_header_case"] # [cfg (all (any (feature = "client" , feature = "server") , feature = "http1"))] # [derive (Clone , Debug)] pub (crate) struct HeaderCaseMap (HeaderMap < Bytes >) ;
    };
}

HeaderCaseMap!()