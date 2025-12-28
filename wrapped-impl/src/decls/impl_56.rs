macro_rules! deps {
    () => {
        InferredBounds!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl InferredBounds { pub fn new () -> Self { InferredBounds { bounds : Map :: new () , order : Vec :: new () , } } pub fn insert (& mut self , ty : impl ToTokens , bound : impl ToTokens) { let ty = ty . to_token_stream () ; let bound = bound . to_token_stream () ; let entry = self . bounds . entry (ty . to_string ()) ; if let Entry :: Vacant (_) = entry { self . order . push (ty) ; } let (set , tokens) = entry . or_default () ; if set . insert (bound . to_string ()) { tokens . push (bound) ; } } pub fn augment_where_clause (& self , generics : & Generics) -> WhereClause { let mut generics = generics . clone () ; let where_clause = generics . make_where_clause () ; for ty in & self . order { let (_set , bounds) = & self . bounds [& ty . to_string ()] ; where_clause . predicates . push (parse_quote ! (# ty : # bounds)) ; } generics . where_clause . unwrap () } }
    };
}

impl_56!()