macro_rules! CTRL_C_RECEIVED {
    () => {
        # [doc = " `rustc_driver::main` installs a handler that will set this to `true` if"] # [doc = " the compiler has been sent a request to shut down, such as by a Ctrl-C."] # [doc = " This static lives here because it is only read by the interpreter."] pub static CTRL_C_RECEIVED : AtomicBool = AtomicBool :: new (false) ;
    };
}

CTRL_C_RECEIVED!()