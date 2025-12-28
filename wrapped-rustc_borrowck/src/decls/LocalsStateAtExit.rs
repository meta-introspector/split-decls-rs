macro_rules! LocalsStateAtExit {
    () => {
        pub enum LocalsStateAtExit { AllAreInvalidated , SomeAreInvalidated { has_storage_dead_or_moved : DenseBitSet < Local > } , }
    };
}

LocalsStateAtExit!()