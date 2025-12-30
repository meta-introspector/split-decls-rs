// Generated macro for SyncContext (struct)
macro_rules! Depcrate_syncSyncContext {
() => {
// Module: crate::sync
// Provides: {"SyncContext"}
// Dependencies: {}
# [doc = " Sync actor execution context. This is used instead of impl Actor for your Actor"] # [doc = " instead of Context, if you intend this actor to run in a [`SyncArbiter`]."] # [doc = ""] # [doc = " Unlike Context, an Actor that uses a [`SyncContext`] can not be stopped"] # [doc = " by calling `stop` or `terminate`: Instead, these trigger a restart of"] # [doc = " the Actor. Similar, returning `false` from `fn stopping` can not prevent"] # [doc = " the restart or termination of the Actor."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use actix::prelude::*;"] # [doc = ""] # [doc = " # struct Fibonacci(pub u32);"] # [doc = ""] # [doc = " # impl Message for Fibonacci {"] # [doc = " #     type Result = Result<u64, ()>;"] # [doc = " # }"] # [doc = ""] # [doc = " struct SyncActor;"] # [doc = ""] # [doc = " impl Actor for SyncActor {"] # [doc = "     // It's important to note that you use \"SyncContext\" here instead of \"Context\"."] # [doc = "     type Context = SyncContext<Self>;"] # [doc = " }"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " # }"] # [doc = " ```"] pub struct SyncContext < A > where A : Actor < Context = SyncContext < A > > , { act : Option < A > , queue : cb_channel :: Receiver < Envelope < A > > , stopping : bool , state : ActorState , factory : Arc < dyn Fn () -> A > , address : AddressSenderProducer < A > , }
};
}
