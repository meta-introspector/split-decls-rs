macro_rules! fence_acquire {
    () => {
        # [inline] fn fence_acquire (a : & AtomicUsize) { if cfg ! (tsan_enabled) { let _ = a . load (Ordering :: Acquire) ; } else { fence (Ordering :: Acquire) ; } }
    };
}

fence_acquire!();