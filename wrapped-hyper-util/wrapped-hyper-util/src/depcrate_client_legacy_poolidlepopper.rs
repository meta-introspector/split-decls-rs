// Generated macro for IdlePopper (struct)
macro_rules! Depcrate_client_legacy_poolIdlePopper {
() => {
// Module: crate::client::legacy::pool
// Provides: {"IdlePopper"}
// Dependencies: {}
# [doc = " Pop off this list, looking for a usable connection that hasn't expired."] struct IdlePopper < 'a , T , K > { key : & 'a K , list : & 'a mut Vec < Idle < T > > , }
};
}
