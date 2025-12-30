// Generated macro for impl_82 (impl)
macro_rules! Depcrateimpl_82 {
() => {
// Module: crate
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > State < T > { fn is_notified (& self) -> bool { matches ! (self , Self :: Notified { .. } | Self :: NotifiedTaken) } # [doc = " If this state was notified, return the tag associated with the notification."] # [allow (unused)] fn notified (self) -> Option < T > { match self { Self :: Notified { tag , .. } => Some (tag) , Self :: NotifiedTaken => panic ! ("listener was already notified but taken") , _ => None , } } }
};
}
