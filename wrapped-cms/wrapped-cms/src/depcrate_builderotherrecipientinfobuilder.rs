// Generated macro for OtherRecipientInfoBuilder (struct)
macro_rules! Depcrate_builderOtherRecipientInfoBuilder {
() => {
// Module: crate::builder
// Provides: {"OtherRecipientInfoBuilder"}
// Dependencies: {}
# [doc = " Builds an `OtherRecipientInfo` according to RFC 5652 § 6."] # [doc = " This type makes no assumption about the encryption method or the needed information."] pub struct OtherRecipientInfoBuilder < R : ? Sized > { # [doc = " Identifies the key management technique."] pub ori_type : ObjectIdentifier , # [doc = " Contains the protocol data elements needed by a recipient using the identified key"] # [doc = " management technique"] pub ori_value : Any , _rng : PhantomData < R > , }
};
}
