macro_rules! macro_1080 {
    () => {
        pin_project ! { # [doc = " Wrap a writer, like [`BufWriter`] does, but prioritizes buffering lines"] # [doc = ""] # [doc = " This was written based on `std::io::LineWriter` which goes into further details"] # [doc = " explaining the code."] # [doc = ""] # [doc = " Buffering is actually done using `BufWriter`. This class will leverage `BufWriter`"] # [doc = " to write on-each-line."] # [derive (Debug)] pub struct LineWriter < W : AsyncWrite > { # [pin] buf_writer : BufWriter < W >, } }
    };
}

macro_1080!()