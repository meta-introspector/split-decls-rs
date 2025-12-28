macro_rules! Symbol {
    () => {
        # [doc = " A trait representing the resolution of a symbol in a file."] # [doc = ""] # [doc = " This trait is yielded as a trait object to the closure given to the"] # [doc = " `backtrace::resolve` function, and it is virtually dispatched as it's"] # [doc = " unknown which implementation is behind it."] # [doc = ""] # [doc = " A symbol can give contextual information about a function, for example the"] # [doc = " name, filename, line number, precise address, etc. Not all information is"] # [doc = " always available in a symbol, however, so all methods return an `Option`."] pub struct Symbol { inner : imp :: Symbol < 'static > , }
    };
}

Symbol!()