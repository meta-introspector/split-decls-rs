macro_rules! NoopSpawner {
    () => {
        # [doc = " An implementation of [`Spawn`](futures_task::Spawn) that"] # [doc = " discards spawned futures when used."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures_test::task::NoopSpawner;"] # [doc = ""] # [doc = " let spawner = NoopSpawner::new();"] # [doc = " spawner.spawn(async { }).unwrap();"] # [doc = " ```"] # [derive (Debug)] pub struct NoopSpawner { _reserved : () , }
    };
}

NoopSpawner!()