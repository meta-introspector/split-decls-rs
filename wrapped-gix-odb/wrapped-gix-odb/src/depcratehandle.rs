// Generated macro for Handle (type)
macro_rules! DepcrateHandle {
() => {
// Module: crate
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " A thread-local handle to access any object."] pub type Handle = Cache < store :: Handle < OwnShared < Store > > > ;
};
}
