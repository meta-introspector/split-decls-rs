// Generated macro for Describe (struct)
macro_rules! Depcrate_describeDescribe {
() => {
// Module: crate::describe
// Provides: {"Describe"}
// Dependencies: {}
# [doc = " The result of a `describe` operation on either an `Describe` or a"] # [doc = " `Repository`."] pub struct Describe < 'repo > { raw : * mut raw :: git_describe_result , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
