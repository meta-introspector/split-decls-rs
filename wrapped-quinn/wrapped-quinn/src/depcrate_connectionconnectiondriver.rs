// Generated macro for ConnectionDriver (struct)
macro_rules! Depcrate_connectionConnectionDriver {
() => {
// Module: crate::connection
// Provides: {"ConnectionDriver"}
// Dependencies: {}
# [doc = " A future that drives protocol logic for a connection"] # [doc = ""] # [doc = " This future handles the protocol logic for a single connection, routing events from the"] # [doc = " `Connection` API object to the `Endpoint` task and the related stream-related interfaces."] # [doc = " It also keeps track of outstanding timeouts for the `Connection`."] # [doc = ""] # [doc = " If the connection encounters an error condition, this future will yield an error. It will"] # [doc = " terminate (yielding `Ok(())`) if the connection was closed without error. Unlike other"] # [doc = " connection-related futures, this waits for the draining period to complete to ensure that"] # [doc = " packets still in flight from the peer are handled gracefully."] # [must_use = "connection drivers must be spawned for their connections to function"] # [derive (Debug)] struct ConnectionDriver (ConnectionRef) ;
};
}
