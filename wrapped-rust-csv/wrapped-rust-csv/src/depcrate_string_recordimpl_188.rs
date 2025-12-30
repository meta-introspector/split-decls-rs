// Generated macro for impl_188 (impl)
macro_rules! Depcrate_string_recordimpl_188 {
() => {
// Module: crate::string_record
// Provides: {"impl_188"}
// Dependencies: {}
impl < T : AsRef < str > > FromIterator < T > for StringRecord { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> StringRecord { let mut record = StringRecord :: new () ; record . extend (iter) ; record } }
};
}
