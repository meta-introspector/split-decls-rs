macro_rules! deps {
    () => {
        WriterThread!();
    };
}

macro_rules! ReadFilterOutput {
    () => {
        deps!();
        # [doc = " A utility type to facilitate streaming the output of a filter process."] struct ReadFilterOutput { inner : Option < std :: process :: ChildStdout > , # [doc = " The child is present if we need its exit code to be positive."] child : Option < (std :: process :: Child , std :: process :: Command) > , # [doc = " The thread writing to stdin, if any. Must be joined when reading is done."] write_thread : Option < WriterThread > , }
    };
}

ReadFilterOutput!()