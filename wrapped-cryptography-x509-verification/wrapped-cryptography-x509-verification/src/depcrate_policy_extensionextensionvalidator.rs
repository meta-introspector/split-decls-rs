// Generated macro for ExtensionValidator (enum)
macro_rules! Depcrate_policy_extensionExtensionValidator {
() => {
// Module: crate::policy::extension
// Provides: {"ExtensionValidator"}
// Dependencies: {}
# [doc = " Represents different validation states for an extension."] # [derive (Clone)] pub enum ExtensionValidator < 'cb , B : CryptoOps > { # [doc = " The extension MUST NOT be present."] NotPresent { oid : asn1 :: ObjectIdentifier } , # [doc = " The extension MUST be present."] Present { oid : asn1 :: ObjectIdentifier , # [doc = " The extension's criticality."] criticality : Criticality , # [doc = " An optional validator over the extension's inner contents, with"] # [doc = " the surrounding `Policy` as context."] validator : Option < PresentExtensionValidatorCallback < 'cb , B > > , } , # [doc = " The extension MAY be present; the interior validator is"] # [doc = " always called if supplied, including if the extension is not present."] MaybePresent { oid : asn1 :: ObjectIdentifier , criticality : Criticality , validator : Option < MaybeExtensionValidatorCallback < 'cb , B > > , } , }
};
}
