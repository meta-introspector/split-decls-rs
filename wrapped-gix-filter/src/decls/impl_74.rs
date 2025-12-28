macro_rules! deps {
    () => {
        Client!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Client { # [doc = " Return the child handle of the running process."] # [doc = ""] # [doc = " Note that this will naturally close input and output handles, which is a signal for the child process to shutdown."] pub fn into_child (self) -> std :: process :: Child { self . child } }
    };
}

impl_74!();