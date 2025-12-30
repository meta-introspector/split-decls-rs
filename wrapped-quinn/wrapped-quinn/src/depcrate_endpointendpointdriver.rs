// Generated macro for EndpointDriver (struct)
macro_rules! Depcrate_endpointEndpointDriver {
() => {
// Module: crate::endpoint
// Provides: {"EndpointDriver"}
// Dependencies: {}
# [doc = " A future that drives IO on an endpoint"] # [doc = ""] # [doc = " This task functions as the switch point between the UDP socket object and the"] # [doc = " `Endpoint` responsible for routing datagrams to their owning `Connection`."] # [doc = " In order to do so, it also facilitates the exchange of different types of events"] # [doc = " flowing between the `Endpoint` and the tasks managing `Connection`s. As such,"] # [doc = " running this task is necessary to keep the endpoint's connections running."] # [doc = ""] # [doc = " `EndpointDriver` futures terminate when all clones of the `Endpoint` have been dropped, or when"] # [doc = " an I/O error occurs."] # [must_use = "endpoint drivers must be spawned for I/O to occur"] # [derive (Debug)] pub (crate) struct EndpointDriver (pub (crate) EndpointRef) ;
};
}
