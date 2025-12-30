// Generated macro for NIC (static)
macro_rules! Depcrate_executor_networkNIC {
() => {
// Module: crate::executor::network
// Provides: {"NIC"}
// Dependencies: {}
pub (crate) static NIC : InterruptTicketMutex < NetworkState < '_ > > = InterruptTicketMutex :: new (NetworkState :: Missing) ;
};
}
