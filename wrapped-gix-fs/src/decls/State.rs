macro_rules! State {
    () => {
        enum State { CurrentlyCreatingDirectories , SearchingUpwardsForExistingDirectory , }
    };
}

State!();