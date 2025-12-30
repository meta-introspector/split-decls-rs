// Generated macro for Statuses (struct)
macro_rules! Depcrate_statusStatuses {
() => {
// Module: crate::status
// Provides: {"Statuses"}
// Dependencies: {}
# [doc = " A container for a list of status information about a repository."] # [doc = ""] # [doc = " Each instance appears as if it were a collection, having a length and"] # [doc = " allowing indexing, as well as providing an iterator."] pub struct Statuses < 'repo > { raw : * mut raw :: git_status_list , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
