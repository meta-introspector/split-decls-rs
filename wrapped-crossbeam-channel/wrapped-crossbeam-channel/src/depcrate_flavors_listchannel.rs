// Generated macro for Channel (struct)
macro_rules! Depcrate_flavors_listChannel {
() => {
// Module: crate::flavors::list
// Provides: {"Channel"}
// Dependencies: {}
# [doc = " Unbounded channel implemented as a linked list."] # [doc = ""] # [doc = " Each message sent into the channel is assigned a sequence number, i.e. an index. Indices are"] # [doc = " represented as numbers of type `usize` and wrap on overflow."] # [doc = ""] # [doc = " Consecutive messages are grouped into blocks in order to put less pressure on the allocator and"] # [doc = " improve cache efficiency."] pub (crate) struct Channel < T > { # [doc = " The head of the channel."] head : CachePadded < Position < T > > , # [doc = " The tail of the channel."] tail : CachePadded < Position < T > > , # [doc = " Receivers waiting while the channel is empty and not disconnected."] receivers : SyncWaker , # [doc = " Indicates that dropping a `Channel<T>` may drop messages of type `T`."] _marker : PhantomData < T > , }
};
}
