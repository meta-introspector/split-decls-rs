// Generated macro for impl_516 (impl)
macro_rules! Depcrate_common_rangeimpl_516 {
() => {
// Module: crate::common::range
// Provides: {"impl_516"}
// Dependencies: {}
impl Range { # [doc = " Creates a `Range` header from bounds."] pub fn bytes (bounds : impl RangeBounds < u64 >) -> Result < Self , InvalidRange > { let v = match (bounds . start_bound () , bounds . end_bound ()) { (Bound :: Included (start) , Bound :: Included (end)) => format ! ("bytes={}-{}" , start , end) , (Bound :: Included (start) , Bound :: Excluded (& end)) => { format ! ("bytes={}-{}" , start , end - 1) } (Bound :: Included (start) , Bound :: Unbounded) => format ! ("bytes={}-" , start) , _ => return Err (InvalidRange { _inner : () }) , } ; Ok (Range (HeaderValue :: from_str (& v) . unwrap ())) } # [doc = " Iterate the range sets as a tuple of bounds, if valid with length."] # [doc = ""] # [doc = " The length of the content is passed as an argument, and all ranges"] # [doc = " that can be satisfied will be iterated."] pub fn satisfiable_ranges (& self , len : u64 ,) -> impl Iterator < Item = (Bound < u64 > , Bound < u64 >) > + '_ { let s = self . 0 . to_str () . expect ("valid string checked in Header::decode()") ; s ["bytes=" . len () ..] . split (',') . filter_map (move | spec | { let mut iter = spec . trim () . splitn (2 , '-') ; let start = parse_bound (iter . next () ?) ? ; let end = parse_bound (iter . next () ?) ? ; if let Bound :: Unbounded = start { if let Bound :: Included (end) = end { if len < end { return None ; } return Some ((Bound :: Included (len - end) , Bound :: Unbounded)) ; } } Some ((start , end)) }) } }
};
}
