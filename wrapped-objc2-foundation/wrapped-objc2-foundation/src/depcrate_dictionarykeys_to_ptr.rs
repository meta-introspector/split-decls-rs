// Generated macro for keys_to_ptr (function)
macro_rules! Depcrate_dictionarykeys_to_ptr {
() => {
// Module: crate::dictionary
// Provides: {"keys_to_ptr"}
// Dependencies: {}
# [cfg (feature = "NSObject")] fn keys_to_ptr < CopiedKey > (keys : & [& CopiedKey]) -> * mut NonNull < ProtocolObject < dyn NSCopying > > where CopiedKey : Message + NSCopying , { let keys : * mut NonNull < CopiedKey > = util :: ref_ptr_cast_const (keys . as_ptr ()) ; let keys : * mut NonNull < ProtocolObject < dyn NSCopying > > = keys . cast () ; keys }
};
}
