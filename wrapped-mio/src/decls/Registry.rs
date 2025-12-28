macro_rules! Registry {
    () => {
        # [doc = " Registers I/O resources."] pub struct Registry { selector : sys :: Selector , # [doc = " Whether this selector currently has an associated waker."] # [cfg (all (debug_assertions , not (target_os = "wasi")))] has_waker : Arc < AtomicBool > , }
    };
}

Registry!();