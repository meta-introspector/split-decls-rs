// Generated macro for ExtensionPolicy (struct)
macro_rules! Depcrate_policy_extensionExtensionPolicy {
() => {
// Module: crate::policy::extension
// Provides: {"ExtensionPolicy"}
// Dependencies: {}
# [derive (Clone)] pub struct ExtensionPolicy < 'cb , B : CryptoOps > { pub authority_information_access : ExtensionValidator < 'cb , B > , pub authority_key_identifier : ExtensionValidator < 'cb , B > , pub subject_key_identifier : ExtensionValidator < 'cb , B > , pub key_usage : ExtensionValidator < 'cb , B > , pub subject_alternative_name : ExtensionValidator < 'cb , B > , pub basic_constraints : ExtensionValidator < 'cb , B > , pub name_constraints : ExtensionValidator < 'cb , B > , pub extended_key_usage : ExtensionValidator < 'cb , B > , }
};
}
