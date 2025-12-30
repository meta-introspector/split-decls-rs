// Generated macro for LocalPool (struct)
macro_rules! Depcrate_local_poolLocalPool {
() => {
// Module: crate::local_pool
// Provides: {"LocalPool"}
// Dependencies: {}
# [doc = " A single-threaded task pool for polling futures to completion."] # [doc = ""] # [doc = " This executor allows you to multiplex any number of tasks onto a single"] # [doc = " thread. It's appropriate to poll strictly I/O-bound futures that do very"] # [doc = " little work in between I/O actions."] # [doc = ""] # [doc = " To get a handle to the pool that implements"] # [doc = " [`Spawn`](futures_task::Spawn), use the"] # [doc = " [`spawner()`](LocalPool::spawner) method. Because the executor is"] # [doc = " single-threaded, it supports a special form of task spawning for non-`Send`"] # [doc = " futures, via [`spawn_local_obj`](futures_task::LocalSpawn::spawn_local_obj)."] # [derive (Debug)] pub struct LocalPool { pool : FuturesUnordered < LocalFutureObj < 'static , () > > , incoming : Rc < Incoming > , }
};
}
