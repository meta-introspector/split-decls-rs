// Generated macro for SnapshotValue (enum)
macro_rules! Depcrate_runtimeSnapshotValue {
() => {
// Module: crate::runtime
// Provides: {"SnapshotValue"}
// Dependencies: {}
pub enum SnapshotValue < 'a > { # [doc = " A text snapshot that gets stored along with the metadata in the same file."] FileText { name : SnapshotName < 'a > , # [doc = " The new generated value to compare against any previously approved content."] content : & 'a str , } , # [doc = " An inline snapshot."] InlineText { # [doc = " The reference content from the macro invocation that will be compared against."] reference_content : & 'a str , # [doc = " The new generated value to compare against any previously approved content."] content : & 'a str , } , # [doc = " A binary snapshot that gets stored as a separate file next to the metadata file."] Binary { name : SnapshotName < 'a > , # [doc = " The new generated value to compare against any previously approved content."] content : Vec < u8 > , # [doc = " The extension of the separate file."] extension : & 'a str , } , }
};
}
