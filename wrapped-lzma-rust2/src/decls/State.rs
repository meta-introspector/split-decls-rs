macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! State {
    () => {
        deps!();
        enum State { # [doc = " Actively reading from the inner reader and sending work to threads."] Reading , # [doc = " The inner reader has reached EOF. We are now waiting for the remaining"] # [doc = " work to be completed by the worker threads."] Draining , # [doc = " All data has been decompressed and returned. The stream is exhausted."] Finished , # [doc = " A fatal error occurred in either the reader or a worker thread."] Error , }
    };
}

State!()