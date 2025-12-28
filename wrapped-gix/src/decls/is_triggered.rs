macro_rules! is_triggered {
    () => {
        # [doc = " Returns true if an interrupt is requested."] pub fn is_triggered () -> bool { IS_INTERRUPTED . load (Ordering :: Relaxed) }
    };
}

is_triggered!();