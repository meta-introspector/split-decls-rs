// Generated macro for EntryPointCleaner (struct)
macro_rules! Depcrate_test_harnessEntryPointCleaner {
() => {
// Module: crate::test_harness
// Provides: {"EntryPointCleaner"}
// Dependencies: {}
# [doc = " A folder used to remove any entry points (like fn main) because the harness"] # [doc = " coroutine will provide its own"] struct EntryPointCleaner < 'a > { sess : & 'a Session , depth : usize , def_site : Span , }
};
}
