// Generated macro for impl_75 (impl)
macro_rules! Depcrate_generatedimpl_75 {
() => {
// Module: crate::generated
// Provides: {"impl_75"}
// Dependencies: {}
impl XCTestObservationCenter { extern_methods ! (# [doc = " Returns: The shared XCTestObservationCenter singleton instance."] # [unsafe (method (sharedTestObservationCenter))] # [unsafe (method_family = none)] pub fn sharedTestObservationCenter () -> Retained < XCTestObservationCenter >; # [doc = " Register an object conforming to XCTestObservation as an observer for the current test session. Observers may be added"] # [doc = " at any time, but will not receive events that occurred before they were registered. The observation center maintains a strong"] # [doc = " reference to observers."] # [doc = ""] # [doc = " Events may be delivered to observers in any order - given observers A and B, A may be notified of a test failure before"] # [doc = " or after B. Any ordering dependencies or serialization requirements must be managed by clients."] # [unsafe (method (addTestObserver :))] # [unsafe (method_family = none)] pub fn addTestObserver (& self , test_observer : & ProtocolObject < dyn XCTestObservation >) ; # [doc = " Unregister an object conforming to XCTestObservation as an observer for the current test session."] # [unsafe (method (removeTestObserver :))] # [unsafe (method_family = none)] pub fn removeTestObserver (& self , test_observer : & ProtocolObject < dyn XCTestObservation >) ;) ; }
};
}
