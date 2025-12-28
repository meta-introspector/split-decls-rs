macro_rules! unsync_load {
    () => {
        # [inline (always)] pub (crate) unsafe fn unsync_load (u : * const atomic :: AtomicU32) -> u32 { * u . cast :: < u32 > () }
    };
}

unsync_load!()