macro_rules! IncOnNewAndDecOnDrop {
    () => {
        struct IncOnNewAndDecOnDrop < 'a > (& 'a AtomicU16) ;
    };
}

IncOnNewAndDecOnDrop!();