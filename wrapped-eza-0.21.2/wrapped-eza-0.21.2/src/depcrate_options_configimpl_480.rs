// Generated macro for impl_480 (impl)
macro_rules! Depcrate_options_configimpl_480 {
() => {
// Module: crate::options::config
// Provides: {"impl_480"}
// Dependencies: {}
impl < R , S , T > FromOverride < HashMap < R , S > > for HashMap < R , T > where T : FromOverride < S > , R : Clone + Eq + std :: hash :: Hash , T : Clone + Eq + Default , { fn from (value : HashMap < R , S > , default : HashMap < R , T >) -> HashMap < R , T > { let mut result = default . clone () ; for (r , s) in value { let t = match default . get (& r) { Some (t) => t . clone () , None => T :: default () , } ; result . insert (r , FromOverride :: from (s , t)) ; } result } }
};
}
