macro_rules! deps {
    () => {
        Testable!();
        QuickCheck!();
    };
}

macro_rules! quickcheck {
    () => {
        deps!();
        # [doc = " Convenience function for running `QuickCheck`."] # [doc = ""] # [doc = " This is an alias for `QuickCheck::new().quickcheck(f)`."] pub fn quickcheck < A : Testable > (f : A) { QuickCheck :: new () . quickcheck (f) }
    };
}

quickcheck!()