macro_rules! WriteWithProgress {
    () => {
        pub struct WriteWithProgress < 'a , T > { pub inner : T , pub progress : & 'a AtomicUsize , }
    };
}

WriteWithProgress!()