macro_rules! ByteLines {
    () => {
        # [doc = " An iterator over lines from an instance of"] # [doc = " [`std::io::BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)."] # [doc = ""] # [doc = " This iterator is generally created by calling the"] # [doc = " [`byte_lines`](trait.BufReadExt.html#method.byte_lines)"] # [doc = " method on the"] # [doc = " [`BufReadExt`](trait.BufReadExt.html)"] # [doc = " trait."] # [derive (Debug)] pub struct ByteLines < B > { buf : B , }
    };
}

ByteLines!()