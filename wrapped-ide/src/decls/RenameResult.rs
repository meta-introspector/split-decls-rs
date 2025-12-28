macro_rules! RenameResult {
    () => {
        type RenameResult < T > = Result < T , RenameError > ;
    };
}

RenameResult!();