macro_rules! ErrnoSentinel {
    () => {
        # [doc = " The sentinel value indicates that a function failed and more detailed"] # [doc = " information about the error can be found in `errno`"] pub trait ErrnoSentinel : Sized { fn sentinel () -> Self ; }
    };
}

ErrnoSentinel!();