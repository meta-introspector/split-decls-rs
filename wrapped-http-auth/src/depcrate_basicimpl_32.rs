// Generated macro for impl_32 (impl)
macro_rules! Depcrate_basicimpl_32 {
() => {
// Module: crate::basic
// Provides: {"impl_32"}
// Dependencies: {}
impl TryFrom < & ChallengeRef < '_ > > for BasicClient { type Error = String ; fn try_from (value : & ChallengeRef < '_ >) -> Result < Self , Self :: Error > { if ! value . scheme . eq_ignore_ascii_case ("Basic") { return Err (format ! ("BasicClient doesn't support challenge scheme {:?}" , value . scheme)) ; } let mut realm = None ; for (k , v) in & value . params { if k . eq_ignore_ascii_case ("realm") { realm = Some (v . to_unescaped ()) ; } } let realm = realm . ok_or ("missing required parameter realm") ? ; Ok (BasicClient { realm : realm . into_boxed_str () , }) } }
};
}
