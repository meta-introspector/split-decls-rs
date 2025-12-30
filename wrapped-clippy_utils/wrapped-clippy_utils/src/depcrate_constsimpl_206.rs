// Generated macro for impl_206 (impl)
macro_rules! Depcrate_constsimpl_206 {
() => {
// Module: crate::consts
// Provides: {"impl_206"}
// Dependencies: {}
impl Ord for FullInt { fn cmp (& self , other : & Self) -> Ordering { use FullInt :: { S , U } ; fn cmp_s_u (s : i128 , u : u128) -> Ordering { u128 :: try_from (s) . map_or (Ordering :: Less , | x | x . cmp (& u)) } match (* self , * other) { (S (s) , S (o)) => s . cmp (& o) , (U (s) , U (o)) => s . cmp (& o) , (S (s) , U (o)) => cmp_s_u (s , o) , (U (s) , S (o)) => cmp_s_u (o , s) . reverse () , } } }
};
}
