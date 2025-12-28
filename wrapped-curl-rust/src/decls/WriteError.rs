macro_rules! WriteError {
    () => {
        # [doc = " Possible error codes that can be returned from the `write_function` callback."] # [non_exhaustive] # [derive (Debug)] pub enum WriteError { # [doc = " Indicates that reading should be paused until `unpause` is called."] Pause , }
    };
}

WriteError!()