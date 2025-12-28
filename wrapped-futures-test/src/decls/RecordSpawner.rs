macro_rules! RecordSpawner {
    () => {
        # [doc = " An implementation of [`Spawn`](futures_task::Spawn) that records"] # [doc = " any [`Future`](futures_core::future::Future)s spawned on it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures_test::task::RecordSpawner;"] # [doc = ""] # [doc = " let recorder = RecordSpawner::new();"] # [doc = " recorder.spawn(async { }).unwrap();"] # [doc = " assert_eq!(recorder.spawned().len(), 1);"] # [doc = " ```"] # [derive (Debug , Default)] pub struct RecordSpawner { spawned : RefCell < Vec < FutureObj < 'static , () > > > , }
    };
}

RecordSpawner!()