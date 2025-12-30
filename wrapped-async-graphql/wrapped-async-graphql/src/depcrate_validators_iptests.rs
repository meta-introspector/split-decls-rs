// Generated macro for tests (module)
macro_rules! Depcrate_validators_iptests {
() => {
// Module: crate::validators::ip
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_ip () { assert ! (ip (& "1.1.1.1" . to_string ()) . is_ok ()) ; assert ! (ip (& "255.0.0.0" . to_string ()) . is_ok ()) ; assert ! (ip (& "256.1.1.1" . to_string ()) . is_err ()) ; assert ! (ip (& "fe80::223:6cff:fe8a:2e8a" . to_string ()) . is_ok ()) ; assert ! (ip (& "::ffff:254.42.16.14" . to_string ()) . is_ok ()) ; assert ! (ip (& "2a02::223:6cff :fe8a:2e8a" . to_string ()) . is_err ()) ; } }
};
}
