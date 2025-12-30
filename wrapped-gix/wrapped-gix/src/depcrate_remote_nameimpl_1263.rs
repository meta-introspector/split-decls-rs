// Generated macro for impl_1263 (impl)
macro_rules! Depcrate_remote_nameimpl_1263 {
() => {
// Module: crate::remote::name
// Provides: {"impl_1263"}
// Dependencies: {}
impl From < BString > for Name < 'static > { fn from (name : BString) -> Self { Self :: try_from (Cow :: Owned (name)) . expect ("String is never illformed") } }
};
}
