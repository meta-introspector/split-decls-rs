macro_rules! deps {
    () => {
        IoRead!();
        LineColIterator!();
        Read!();
    };
}

macro_rules! impl_575 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < R > IoRead < R > where R : io :: Read , { # [doc = " Create a JSON input source to read from a std::io input stream."] # [doc = ""] # [doc = " When reading from a source against which short reads are not efficient, such"] # [doc = " as a [`File`], you will want to apply your own buffering because serde_json"] # [doc = " will not buffer the input. See [`std::io::BufReader`]."] # [doc = ""] # [doc = " [`File`]: std::fs::File"] pub fn new (reader : R) -> Self { IoRead { iter : LineColIterator :: new (reader . bytes ()) , ch : None , # [cfg (feature = "raw_value")] raw_buffer : None , } } }
    };
}

impl_575!()