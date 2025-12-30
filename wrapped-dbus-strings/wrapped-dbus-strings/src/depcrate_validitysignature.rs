// Generated macro for signature (function)
macro_rules! Depcrate_validitysignature {
() => {
// Module: crate::validity
// Provides: {"signature"}
// Dependencies: {}
# [test] fn signature () { assert ! (is_valid_signature_single (b"") . is_err ()) ; assert ! (is_valid_signature_single (b"i") . is_ok ()) ; assert ! (is_valid_signature_single (b"ii") . is_err ()) ; assert ! (is_valid_signature_single (b"vi") . is_err ()) ; assert ! (is_valid_signature_single (b"g") . is_ok ()) ; assert ! (is_valid_signature_single (b"{ss}") . is_err ()) ; assert ! (is_valid_signature_single (b"ad") . is_ok ()) ; assert ! (is_valid_signature_single (b"a{ss}") . is_ok ()) ; assert ! (is_valid_signature_single (b"a{vs}") . is_err ()) ; assert ! (is_valid_signature_single (b"a{ss}i") . is_err ()) ; assert ! (is_valid_signature_single (b"a{oa{sv}}") . is_ok ()) ; assert ! (is_valid_signature_single (b"v") . is_ok ()) ; assert ! (is_valid_signature_single (b"()") . is_err ()) ; assert ! (is_valid_signature_single (b"(s)") . is_ok ()) ; assert ! (is_valid_signature_single (b"(sa{sv}(i))") . is_ok ()) ; assert ! (is_valid_signature_single (b"(sa{sv}(i)") . is_err ()) ; assert ! (is_valid_signature_single (b"(dbus)") . is_ok ()) ; assert ! (is_valid_signature_multi (b"dbus") . is_ok ()) ; assert ! (is_valid_signature_multi (b"") . is_ok ()) ; assert ! (is_valid_signature_multi (b"dbus)") . is_err ()) ; }
};
}
