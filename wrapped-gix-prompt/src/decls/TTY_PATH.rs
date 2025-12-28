macro_rules! TTY_PATH {
    () => {
        # [doc = " The path to the default TTY on linux"] pub const TTY_PATH : & str = "/dev/tty" ;
    };
}

TTY_PATH!();