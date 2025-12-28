macro_rules! deps {
    () => {
        Client!();
        Key!();
    };
}

macro_rules! Process {
    () => {
        deps!();
        # [doc = " A literal driver process."] pub enum Process < 'a > { # [doc = " A spawned processes to handle a single file"] SingleFile { # [doc = " The child to use as handle for sending and receiving data."] child : std :: process :: Child , # [doc = " The launched command that produced the `child` in the first place"] command : std :: process :: Command , } , # [doc = " A multi-file process which is launched once to handle one or more files by using a custom IO protocol."] MultiFile { # [doc = " A handle to interact with the long-running process."] client : & 'a mut process :: Client , # [doc = " A way to refer to the `client` later if needed."] key : Key , } , }
    };
}

Process!();