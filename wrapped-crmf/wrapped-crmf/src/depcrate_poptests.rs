// Generated macro for tests (module)
macro_rules! Depcrate_poptests {
() => {
// Module: crate::pop
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use der :: { Tag , TagNumber , asn1 :: Ia5String } ; use x509_cert :: ext :: pkix :: name :: GeneralName ; use super :: EncKeyWithIdChoice ; # [test] fn enc_key_with_id_choice_tag () { use der :: Tagged ; let enc_key_choice = EncKeyWithIdChoice :: GeneralName (GeneralName :: DnsName (Ia5String :: new ("test") . expect ("valid Ia5String") ,)) ; assert_eq ! (enc_key_choice . tag () , Tag :: ContextSpecific { constructed : false , number : TagNumber (2) }) ; } }
};
}
