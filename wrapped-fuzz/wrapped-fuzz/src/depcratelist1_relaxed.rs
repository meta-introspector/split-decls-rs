// Generated macro for list1_relaxed (function)
macro_rules! Depcratelist1_relaxed {
() => {
// Module: crate
// Provides: {"list1_relaxed"}
// Dependencies: {}
# [doc = " Parses `1#element` as defined in"] # [doc = " [RFC 7230 section 7](https://datatracker.ietf.org/doc/html/rfc7230#section-7)."] # [doc = ""] # [doc = " > A recipient MUST accept lists that satisfy the following syntax:"] # [doc = " > ```text"] # [doc = " > 1#element => *( \",\" OWS ) element *( OWS \",\" [ OWS element ] )"] # [doc = " > ```"] fn list1_relaxed < 'i , O , F > (f : F) -> impl FnMut (& 'i str) -> nom :: IResult < & 'i str , Vec < O > > where F : nom :: Parser < & 'i str , O , nom :: error :: Error < & 'i str > > , { delimited (many0_count (pair (char (',') , ows)) , list1_relaxed_inner (f) , many0_count (pair (ows , char (','))) ,) }
};
}
