// Generated macro for EmptySubscription (struct)
macro_rules! Depcrate_types_scalarsEmptySubscription {
() => {
// Module: crate::types::scalars
// Provides: {"EmptySubscription"}
// Dependencies: {}
# [doc = " Utillity type to define read-only schemas"] # [doc = ""] # [doc = " If you instantiate `RootNode` with this as the subscription,"] # [doc = " no subscriptions will be generated for the schema."] pub struct EmptySubscription < T : ? Sized = () > (PhantomData < JoinHandle < Box < T > > >) ;
};
}
