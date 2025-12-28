macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! timer {
    () => {
        deps!();
        # [cfg (any (all (target_arch = "wasm32" , feature = "default") ,))] mod timer { use std :: pin :: Pin ; use std :: task :: Poll ; use gloo_timers :: future :: TimeoutFuture ; # [derive (Debug)] pub (crate) struct Timer (TimeoutFuture) ; impl Timer { pub (crate) fn after (dur : std :: time :: Duration) -> Self { let mut timeout_ms = dur . as_millis () as u32 ; if std :: time :: Duration :: from_millis (timeout_ms as u64) < dur { timeout_ms += 1 ; } Timer (TimeoutFuture :: new (timeout_ms)) } } impl std :: future :: Future for Timer { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut std :: task :: Context < '_ >) -> Poll < Self :: Output > { match Pin :: new (& mut self . 0) . poll (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (_) => Poll :: Ready (()) , } } } }
    };
}

timer!()