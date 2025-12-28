macro_rules! deps {
    () => {
        CommandFactory!();
        Error!();
    };
}

macro_rules! format_error {
    () => {
        deps!();
        fn format_error < I : CommandFactory > (err : Error) -> Error { let mut cmd = I :: command () ; err . format (& mut cmd) }
    };
}

format_error!();