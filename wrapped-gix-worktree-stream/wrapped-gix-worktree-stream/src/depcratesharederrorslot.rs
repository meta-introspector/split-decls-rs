// Generated macro for SharedErrorSlot (type)
macro_rules! DepcrateSharedErrorSlot {
() => {
// Module: crate
// Provides: {"SharedErrorSlot"}
// Dependencies: {}
pub (crate) type SharedErrorSlot = Arc < parking_lot :: Mutex < Option < entry :: Error > > > ;
};
}
