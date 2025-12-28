macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " An entry in a stream. Note that they must be consumed fully, by reading from them till exhaustion."] # [doc = ""] # [doc = " ### Drop behaviour"] # [doc = ""] # [doc = " If the entry is dropped without reading it till exhaustion, the stream is tainted and"] # [doc = " [`next_entry()`][Stream::next_entry()] will panic next time it is called."] pub struct Entry < 'a > { # [doc = " The kind of entry at [`relative_path`][Self::relative_path()]."] pub mode : gix_object :: tree :: EntryMode , # [doc = " The hash of the object, uniquely identifying it."] pub id : gix_hash :: ObjectId , # [doc = " Access to our parent"] parent : & 'a mut Stream , # [doc = " The path relative to the repository at which data should be written."] path_buf : Option < BString > , # [doc = " The amount of bytes left to read if the size of bytes to read is known."] # [doc = " It's also our marker to say that we are depleted, which is important to signal to the"] # [doc = " parent stream that we can proceed reading the next entry."] remaining : Option < usize > , }
    };
}

Entry!()