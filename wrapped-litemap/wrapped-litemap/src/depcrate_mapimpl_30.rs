// Generated macro for impl_30 (impl)
macro_rules! Depcrate_mapimpl_30 {
() => {
// Module: crate::map
// Provides: {"impl_30"}
// Dependencies: {}
impl < K , V , S > Index < & '_ K > for LiteMap < K , V , S > where K : Ord , S : Store < K , V > , { type Output = V ; fn index (& self , key : & K) -> & V { # [expect (clippy :: panic)] match self . get (key) { Some (v) => v , None => panic ! ("no entry found for key") , } } }
};
}
