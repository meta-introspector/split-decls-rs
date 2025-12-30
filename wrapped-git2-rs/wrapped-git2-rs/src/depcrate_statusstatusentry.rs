// Generated macro for StatusEntry (struct)
macro_rules! Depcrate_statusStatusEntry {
() => {
// Module: crate::status
// Provides: {"StatusEntry"}
// Dependencies: {}
# [doc = " A structure representing an entry in the `Statuses` structure."] # [doc = ""] # [doc = " Instances are created through the `.iter()` method or the `.get()` method."] pub struct StatusEntry < 'statuses > { raw : * const raw :: git_status_entry , _marker : marker :: PhantomData < & 'statuses DiffDelta < 'statuses > > , }
};
}
