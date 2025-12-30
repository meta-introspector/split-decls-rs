// Generated macro for list0_relaxed (function)
macro_rules! Depcratelist0_relaxed {
() => {
// Module: crate
// Provides: {"list0_relaxed"}
// Dependencies: {}
# [doc = " Parses `#element` as defined in [RFC 7230 section 7](https://datatracker.ietf.org/doc/html/rfc7230#section-7)."] # [doc = ""] # [doc = " > A recipient MUST accept lists that satisfy the following syntax:"] # [doc = " > ```text"] # [doc = " > #element => [ ( \",\" / element ) *( OWS \",\" [ OWS element ] ) ]"] # [doc = " > ```"] # [cfg (test)] fn list0_relaxed < 'i , O , F > (f : F) -> impl FnMut (& 'i str) -> nom :: IResult < & 'i str , Vec < O > > where F : nom :: Parser < & 'i str , O , nom :: error :: Error < & 'i str > > , { delimited (many0_count (pair (char (',') , ows)) , list0_relaxed_inner (f) , many0_count (pair (ows , char (','))) ,) }
};
}
