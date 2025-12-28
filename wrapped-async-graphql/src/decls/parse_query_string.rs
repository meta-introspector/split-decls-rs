macro_rules! deps {
    () => {
        Result!();
        Error!();
        Request!();
        ParseRequestError!();
    };
}

macro_rules! parse_query_string {
    () => {
        deps!();
        # [doc = " Parse a GraphQL request from a query string."] pub fn parse_query_string (input : & str) -> Result < Request , ParseRequestError > { # [derive (Deserialize)] struct RequestSerde { # [serde (default)] pub query : String , pub operation_name : Option < String > , pub variables : Option < String > , pub extensions : Option < String > , } let request : RequestSerde = serde_urlencoded :: from_str (input) . map_err (std :: io :: Error :: other) ? ; let variables = request . variables . map (| data | serde_json :: from_str (& data)) . transpose () . map_err (| err | std :: io :: Error :: other (format ! ("invalid variables: {}" , err))) ? . unwrap_or_default () ; let extensions = request . extensions . map (| data | serde_json :: from_str (& data)) . transpose () . map_err (| err | std :: io :: Error :: other (format ! ("invalid extensions: {}" , err))) ? . unwrap_or_default () ; Ok (Request { operation_name : request . operation_name , variables , extensions , .. Request :: new (request . query) }) }
    };
}

parse_query_string!()