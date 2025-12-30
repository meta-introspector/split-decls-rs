// Generated macro for impl_37 (impl)
macro_rules! Depcrate_util_entityimpl_37 {
() => {
// Module: crate::util::entity
// Provides: {"impl_37"}
// Dependencies: {}
impl super :: TryFromValues for EntityTagRange { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { let flat = FlatCsv :: try_from_values (values) ? ; if flat . value == "*" { Ok (EntityTagRange :: Any) } else { Ok (EntityTagRange :: Tags (flat)) } } }
};
}
