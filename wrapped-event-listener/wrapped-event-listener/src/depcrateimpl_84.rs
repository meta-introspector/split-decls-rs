// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl < T > RegisterResult < T > { # [doc = " Whether or not the listener was notified."] # [doc = ""] # [doc = " Panics if the listener was never inserted into the list."] fn notified (self) -> Option < T > { match self { Self :: Notified (tag) => Some (tag) , Self :: Registered => None , Self :: NeverInserted => panic ! ("{}" , NEVER_INSERTED_PANIC) , } } }
};
}
