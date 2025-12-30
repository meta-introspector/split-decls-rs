// Generated macro for macro_346 (macro)
macro_rules! Depcrate_markermacro_346 {
() => {
// Module: crate::marker
// Provides: {"macro_346"}
// Dependencies: {}
impl_dyn_sync ! ([std :: sync :: atomic :: AtomicPtr < T > where T] [std :: sync :: OnceLock < T > where T : DynSend + DynSync] [std :: sync :: Mutex < T > where T : ? Sized + DynSend] [std :: sync :: Arc < T > where T : ? Sized + DynSync + DynSend] [std :: sync :: LazyLock < T , F > where T : DynSend + DynSync , F : DynSend] [std :: collections :: HashSet < K , S > where K : DynSync , S : DynSync] [std :: collections :: HashMap < K , V , S > where K : DynSync , V : DynSync , S : DynSync] [std :: collections :: BTreeMap < K , V , A > where K : DynSync , V : DynSync , A : std :: alloc :: Allocator + Clone + DynSync] [Vec < T , A > where T : DynSync , A : std :: alloc :: Allocator + DynSync] [Box < T , A > where T : ? Sized + DynSync , A : std :: alloc :: Allocator + DynSync] [crate :: sync :: RwLock < T > where T : DynSend + DynSync] [crate :: sync :: WorkerLocal < T > where T : DynSend] [crate :: intern :: Interned <'a , T > where 'a , T : DynSync] [crate :: tagged_ptr :: TaggedRef <'a , P , T > where 'a , P : Sync , T : Sync + crate :: tagged_ptr :: Tag] [parking_lot :: lock_api :: Mutex < R , T > where R : DynSync , T : ? Sized + DynSend] [parking_lot :: lock_api :: RwLock < R , T > where R : DynSync , T : ? Sized + DynSend + DynSync] [hashbrown :: HashTable < T > where T : DynSync] [indexmap :: IndexSet < V , S > where V : DynSync , S : DynSync] [indexmap :: IndexMap < K , V , S > where K : DynSync , V : DynSync , S : DynSync] [smallvec :: SmallVec < A > where A : smallvec :: Array + DynSync] [thin_vec :: ThinVec < T > where T : DynSync]) ;
};
}
