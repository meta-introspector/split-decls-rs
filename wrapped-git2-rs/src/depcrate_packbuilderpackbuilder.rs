// Generated macro for PackBuilder (struct)
macro_rules! Depcrate_packbuilderPackBuilder {
() => {
// Module: crate::packbuilder
// Provides: {"PackBuilder"}
// Dependencies: {}
# [doc = " A builder for creating a packfile"] pub struct PackBuilder < 'repo > { raw : * mut raw :: git_packbuilder , _progress : Option < Box < Box < ProgressCb < 'repo > > > > , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
