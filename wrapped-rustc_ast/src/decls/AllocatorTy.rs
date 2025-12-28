macro_rules! AllocatorTy {
    () => {
        pub enum AllocatorTy { Layout , Ptr , ResultPtr , Unit , Usize , }
    };
}

AllocatorTy!();