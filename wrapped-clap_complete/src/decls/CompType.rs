macro_rules! CompType {
    () => {
        # [doc = " Type of completion attempted that caused a completion function to be called"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] # [non_exhaustive] enum CompType { # [doc = " Normal completion"] Normal , # [doc = " List completions after successive tabs"] Successive , # [doc = " List alternatives on partial word completion"] Alternatives , # [doc = " List completions if the word is not unmodified"] Unmodified , # [doc = " Menu completion"] Menu , }
    };
}

CompType!();