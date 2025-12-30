// Generated macro for impl_55 (impl)
macro_rules! Depcrate_util_flat_csvimpl_55 {
() => {
// Module: crate::util::flat_csv
// Provides: {"impl_55"}
// Dependencies: {}
impl < Sep : Separator > TryFromValues for FlatCsv < Sep > { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { let flat = values . collect () ; Ok (flat) } }
};
}
