macro_rules! Stdio {
    () => {
        enum Stdio { Stdin , Stdout , }
    };
}

Stdio!();