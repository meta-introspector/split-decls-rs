// Generated macro for impl_133 (impl)
macro_rules! Depcrate_pemimpl_133 {
() => {
// Module: crate::pem
// Provides: {"impl_133"}
// Dependencies: {}
impl SectionLabel { fn is_end (& self , line : & [u8]) -> bool { let rest = match line . strip_prefix (b"-----END ") { Some (rest) => rest , None => return false , } ; let ty = match self { Self :: Known (kind) => kind . as_slice () , Self :: Unknown (ty) => ty , } ; let rest = match rest . strip_prefix (ty) { Some (rest) => rest , None => return false , } ; rest . starts_with (b"-----") } }
};
}
