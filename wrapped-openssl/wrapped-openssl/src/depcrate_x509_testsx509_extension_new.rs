// Generated macro for x509_extension_new (function)
macro_rules! Depcrate_x509_testsx509_extension_new {
() => {
// Module: crate::x509::tests
// Provides: {"x509_extension_new"}
// Dependencies: {}
# [test] # [allow (deprecated)] fn x509_extension_new () { assert ! (X509Extension :: new (None , None , "crlDistributionPoints" , "section") . is_err ()) ; assert ! (X509Extension :: new (None , None , "proxyCertInfo" , "") . is_err ()) ; assert ! (X509Extension :: new (None , None , "certificatePolicies" , "") . is_err ()) ; assert ! (X509Extension :: new (None , None , "subjectAltName" , "dirName:section") . is_err ()) ; }
};
}
