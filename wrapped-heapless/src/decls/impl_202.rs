macro_rules! deps {
    () => {
        Queue!();
        Storage!();
        QueueView!();
        QueueInner!();
        SealedStorage!();
        OwnedStorage!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl < const N : usize > SealedStorage for OwnedStorage < N > { type Buffer < T > = [T ; N] ; fn len < T > (_ : * const Self :: Buffer < T >) -> usize { N } fn as_ptr < T > (this : * mut Self :: Buffer < T >) -> * mut T { this . cast () } # [cfg (any (feature = "portable-atomic" , all (feature = "mpmc_large" , target_has_atomic = "ptr") , all (not (feature = "mpmc_large") , target_has_atomic = "8")))] fn as_mpmc_view < T > (this : & mpmc :: Queue < T , N >) -> & mpmc :: QueueView < T > where Self : Storage + Sized , { this . as_view_private () } # [cfg (any (feature = "portable-atomic" , all (feature = "mpmc_large" , target_has_atomic = "ptr") , all (not (feature = "mpmc_large") , target_has_atomic = "8")))] fn as_mpmc_mut_view < T > (this : & mut mpmc :: Queue < T , N >) -> & mut mpmc :: QueueView < T > where Self : Storage + Sized , { this . as_view_mut_private () } # [cfg (any (feature = "portable-atomic" , target_has_atomic = "ptr" , has_atomic_load_store))] # [doc = " Convert a `Queue` to a `QueueView`"] fn as_queue_view < T > (this : & spsc :: QueueInner < T , Self >) -> & spsc :: QueueView < T > where Self : Storage + Sized , { this . as_view_private () } # [cfg (any (feature = "portable-atomic" , target_has_atomic = "ptr" , has_atomic_load_store))] # [doc = " Convert a `Queue` to a `QueueView`"] fn as_mut_queue_view < T > (this : & mut spsc :: QueueInner < T , Self >) -> & mut spsc :: QueueView < T > where Self : Storage + Sized , { this . as_mut_view_private () } }
    };
}

impl_202!();