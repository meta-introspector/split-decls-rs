// Generated macro for Notes (struct)
macro_rules! Depcrate_noteNotes {
() => {
// Module: crate::note
// Provides: {"Notes"}
// Dependencies: {}
# [doc = " An iterator over all of the notes within a repository."] pub struct Notes < 'repo > { raw : * mut raw :: git_note_iterator , _marker : marker :: PhantomData < & 'repo Repository > , }
};
}
