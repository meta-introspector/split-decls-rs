// Generated macro for Interest (struct)
macro_rules! Depcrate_interestInterest {
() => {
// Module: crate::interest
// Provides: {"Interest"}
// Dependencies: {}
# [doc = " Interest used in registering."] # [doc = ""] # [doc = " Interest are used in [registering] [`event::Source`]s with [`Poll`], they"] # [doc = " indicate what readiness should be monitored for. For example if a socket is"] # [doc = " registered with [readable] interests and the socket becomes writable, no"] # [doc = " event will be returned from a call to [`poll`]."] # [doc = ""] # [doc = " [registering]: struct.Registry.html#method.register"] # [doc = " [`event::Source`]: ./event/trait.Source.html"] # [doc = " [`Poll`]: struct.Poll.html"] # [doc = " [readable]: struct.Interest.html#associatedconstant.READABLE"] # [doc = " [`poll`]: struct.Poll.html#method.poll"] # [derive (Copy , PartialEq , Eq , Clone , PartialOrd , Ord)] pub struct Interest (NonZeroU8) ;
};
}
