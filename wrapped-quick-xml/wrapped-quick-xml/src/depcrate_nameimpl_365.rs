// Generated macro for impl_365 (impl)
macro_rules! Depcrate_nameimpl_365 {
() => {
// Module: crate::name
// Provides: {"impl_365"}
// Dependencies: {}
impl NamespaceBinding { # [doc = " Get the namespace prefix, bound to this namespace declaration, or `None`,"] # [doc = " if this declaration is for default namespace (`xmlns=\"...\"`)."] # [inline] fn prefix < 'b > (& self , ns_buffer : & 'b [u8]) -> Option < Prefix < 'b > > { if self . prefix_len == 0 { None } else { Some (Prefix (& ns_buffer [self . start .. self . start + self . prefix_len])) } } # [doc = " Gets the namespace name (the URI) slice out of namespace buffer"] # [doc = ""] # [doc = " Returns `None` if namespace for this prefix was explicitly removed from"] # [doc = " scope, using `xmlns[:prefix]=\"\"`"] # [inline] fn namespace < 'ns > (& self , buffer : & 'ns [u8]) -> ResolveResult < 'ns > { if self . value_len == 0 { ResolveResult :: Unbound } else { let start = self . start + self . prefix_len ; ResolveResult :: Bound (Namespace (& buffer [start .. start + self . value_len])) } } }
};
}
