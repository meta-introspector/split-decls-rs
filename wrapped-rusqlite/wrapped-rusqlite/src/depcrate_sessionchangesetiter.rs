// Generated macro for ChangesetIter (struct)
macro_rules! Depcrate_sessionChangesetIter {
() => {
// Module: crate::session
// Provides: {"ChangesetIter"}
// Dependencies: {}
# [doc = " Cursor for iterating over the elements of a changeset"] # [doc = " or patchset."] pub struct ChangesetIter < 'changeset > { phantom : PhantomData < & 'changeset Changeset > , it : * mut ffi :: sqlite3_changeset_iter , item : Option < ChangesetItem > , }
};
}
