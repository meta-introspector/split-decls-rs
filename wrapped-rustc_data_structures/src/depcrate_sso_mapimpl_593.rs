// Generated macro for impl_593 (impl)
macro_rules! Depcrate_sso_mapimpl_593 {
() => {
// Module: crate::sso::map
// Provides: {"impl_593"}
// Dependencies: {}
impl < K : Eq + Hash , V > Extend < (K , V) > for SsoHashMap < K , V > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = (K , V) > , { for (key , value) in iter . into_iter () { self . insert (key , value) ; } } # [inline] fn extend_one (& mut self , (k , v) : (K , V)) { self . insert (k , v) ; } fn extend_reserve (& mut self , additional : usize) { match self { SsoHashMap :: Array (array) => { if SSO_ARRAY_SIZE < (array . len () + additional) { let mut map : FxHashMap < K , V > = array . drain (..) . collect () ; map . extend_reserve (additional) ; * self = SsoHashMap :: Map (map) ; } } SsoHashMap :: Map (map) => map . extend_reserve (additional) , } } }
};
}
