macro_rules! WindowsProducer {
    () => {
        struct WindowsProducer < 'data , T : Sync > { window_size : usize , slice : & 'data [T] , }
    };
}

WindowsProducer!();