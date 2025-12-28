macro_rules! RESET {
    () => {
        # [doc = " The code to send to reset all styles and return to `Style::default()`."] pub static RESET : & str = "\x1B[0m" ;
    };
}

RESET!();