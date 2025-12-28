macro_rules! deps {
    () => {
        NoopSpawner!();
    };
}

macro_rules! noop_spawner_mut {
    () => {
        deps!();
        # [doc = " Get a reference to a singleton instance of [`NoopSpawner`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures_test::task::noop_spawner_mut;"] # [doc = ""] # [doc = " let spawner = noop_spawner_mut();"] # [doc = " spawner.spawn(async { }).unwrap();"] # [doc = " ```"] pub fn noop_spawner_mut () -> & 'static mut NoopSpawner { Box :: leak (Box :: new (NoopSpawner :: new ())) }
    };
}

noop_spawner_mut!()