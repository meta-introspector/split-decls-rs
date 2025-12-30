// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'r > Responder < 'r , 'static > for GraphQLResponse { fn respond_to (self , _ : & 'r rocket :: Request < '_ >) -> response :: Result < 'static > { let body = serde_json :: to_string (& self . 0) . unwrap () ; let mut response = rocket :: Response :: new () ; response . set_header (ContentType :: new ("application" , "json")) ; if self . 0 . is_ok () { if let Some (cache_control) = self . 0 . cache_control () . value () { response . set_header (Header :: new ("cache-control" , cache_control)) ; } } for (name , value) in self . 0 . http_headers_iter () { if let Ok (value) = value . to_str () { response . adjoin_header (Header :: new (name . as_str () . to_string () , value . to_string ())) ; } } response . set_sized_body (body . len () , Cursor :: new (body)) ; Ok (response) } }
};
}
