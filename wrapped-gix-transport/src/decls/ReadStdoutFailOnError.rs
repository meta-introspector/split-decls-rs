macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ReadStdoutFailOnError {
    () => {
        deps!();
        struct ReadStdoutFailOnError { recv : std :: sync :: mpsc :: Receiver < std :: io :: Error > , read : std :: process :: ChildStdout , }
    };
}

ReadStdoutFailOnError!();