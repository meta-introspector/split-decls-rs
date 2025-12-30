// Generated macro for impl_585 (impl)
macro_rules! Depcrate_static_indeximpl_585 {
() => {
// Module: crate::static_index
// Provides: {"impl_585"}
// Dependencies: {}
impl TokenStore { pub fn insert (& mut self , data : TokenStaticData) -> TokenId { let id = TokenId (self . 0 . len ()) ; self . 0 . push (data) ; id } pub fn get_mut (& mut self , id : TokenId) -> Option < & mut TokenStaticData > { self . 0 . get_mut (id . 0) } pub fn get (& self , id : TokenId) -> Option < & TokenStaticData > { self . 0 . get (id . 0) } pub fn iter (self) -> impl Iterator < Item = (TokenId , TokenStaticData) > { self . 0 . into_iter () . enumerate () . map (| (id , data) | (TokenId (id) , data)) } }
};
}
