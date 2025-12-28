macro_rules! sync {
    () => {
        # [cfg (all (test , loom))] pub (crate) mod sync { pub (crate) mod atomic { pub (crate) use loom :: sync :: atomic :: { AtomicPtr , AtomicUsize , Ordering } ; pub (crate) trait AtomicMut < T > { } } }
    };
}

sync!()