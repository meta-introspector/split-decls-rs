// Generated macro for Template (struct)
macro_rules! Depcrate_pkcs8Template {
() => {
// Module: crate::pkcs8
// Provides: {"Template"}
// Dependencies: {}
# [doc = " A template for constructing PKCS#8 documents."] # [doc = ""] # [doc = " Note that this only works for ECC."] pub (crate) struct Template { pub bytes : & 'static [u8] , pub alg_id_range : core :: ops :: Range < usize > , pub curve_id_index : usize , pub private_key_index : usize , }
};
}
