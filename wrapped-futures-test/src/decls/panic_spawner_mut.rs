macro_rules! deps {
    () => {
        PanicSpawner!();
    };
}

macro_rules! panic_spawner_mut {
    () => {
        deps!();
        # [doc = " Get a reference to a singleton instance of [`PanicSpawner`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures_test::task::panic_spawner_mut;"] # [doc = ""] # [doc = " let spawner = panic_spawner_mut();"] # [doc = " spawner.spawn(async { })?; // Will panic"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn panic_spawner_mut () -> & 'static mut PanicSpawner { Box :: leak (Box :: new (PanicSpawner :: new ())) }
    };
}

panic_spawner_mut!();