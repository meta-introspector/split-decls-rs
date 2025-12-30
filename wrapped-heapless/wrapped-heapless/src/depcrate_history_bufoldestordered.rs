// Generated macro for OldestOrdered (struct)
macro_rules! Depcrate_history_bufOldestOrdered {
() => {
// Module: crate::history_buf
// Provides: {"OldestOrdered"}
// Dependencies: {}
# [doc = " Double ended iterator on the underlying buffer ordered from the oldest data"] # [doc = " to the newest."] pub struct OldestOrdered < 'a , T > { inner : core :: iter :: Chain < core :: slice :: Iter < 'a , T > , core :: slice :: Iter < 'a , T > > , }
};
}
