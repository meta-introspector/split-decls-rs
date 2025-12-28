macro_rules! index {
    () => {
        fn index (cnt : u16) -> usize { cnt as usize % MAX_ATOMIC_HISTORY }
    };
}

index!();