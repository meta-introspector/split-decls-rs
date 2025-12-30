// Generated macro for ReadOnlyView (struct)
macro_rules! Depcrate_read_onlyReadOnlyView {
() => {
// Module: crate::read_only
// Provides: {"ReadOnlyView"}
// Dependencies: {}
# [doc = " A read-only view into a `DashMap`. Allows to obtain raw references to the stored values."] pub struct ReadOnlyView < K , V , S = RandomState > { pub (crate) map : DashMap < K , V , S > , }
};
}
