macro_rules! exponential_size {
    () => {
        fn exponential_size (size : & usize) -> Option < usize > { Some (size . saturating_mul (2)) }
    };
}

exponential_size!();