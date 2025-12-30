// Generated macro for Subject (enum)
macro_rules! Depcrate_policySubject {
() => {
// Module: crate::policy
// Provides: {"Subject"}
// Dependencies: {}
# [doc = " Represents a logical certificate \"subject,\" i.e. a principal matching"] # [doc = " one of the names listed in a certificate's `subjectAltNames` extension."] pub enum Subject < 'a > { DNS (DNSName < 'a >) , IP (IPAddress) , }
};
}
