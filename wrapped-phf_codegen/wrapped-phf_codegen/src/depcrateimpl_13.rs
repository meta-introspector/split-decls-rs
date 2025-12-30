// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a , K , V > FromIterator < (K , V) > for Map < 'a , K > where K : Hash + PhfHash + Eq + FmtConst , V : Into < Cow < 'a , str > > , { fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { let mut map = Map :: new () ; for (key , value) in iter { map . entry (key , value) ; } map } }
};
}
