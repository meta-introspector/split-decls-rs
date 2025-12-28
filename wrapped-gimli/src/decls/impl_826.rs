macro_rules! deps {
    () => {
        BaseId!();
    };
}

macro_rules! impl_826 {
    () => {
        deps!();
        # [cfg (debug_assertions)] impl Default for BaseId { fn default () -> Self { use std :: sync :: atomic ; static BASE_ID : atomic :: AtomicUsize = atomic :: AtomicUsize :: new (0) ; BaseId (BASE_ID . fetch_add (1 , atomic :: Ordering :: Relaxed)) } }
    };
}

impl_826!()