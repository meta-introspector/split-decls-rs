macro_rules! DropGuard {
    () => {
        struct DropGuard < T > { ptr : * mut T , len : usize , }
    };
}

DropGuard!();