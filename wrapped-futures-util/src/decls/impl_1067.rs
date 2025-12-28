macro_rules! deps {
    () => {
        Ready!();
        Seek!();
        Current!();
    };
}

macro_rules! impl_1067 {
    () => {
        deps!();
        impl < R : AsyncRead + AsyncSeek > AsyncSeek for BufReader < R > { # [doc = " Seek to an offset, in bytes, in the underlying reader."] # [doc = ""] # [doc = " The position used for seeking with `SeekFrom::Current(_)` is the"] # [doc = " position the underlying reader would be at if the `BufReader` had no"] # [doc = " internal buffer."] # [doc = ""] # [doc = " Seeking always discards the internal buffer, even if the seek position"] # [doc = " would otherwise fall within it. This guarantees that calling"] # [doc = " `.into_inner()` immediately after a seek yields the underlying reader"] # [doc = " at the same position."] # [doc = ""] # [doc = " To seek without discarding the internal buffer, use"] # [doc = " [`BufReader::seek_relative`](BufReader::seek_relative) or"] # [doc = " [`BufReader::poll_seek_relative`](BufReader::poll_seek_relative)."] # [doc = ""] # [doc = " See [`AsyncSeek`](futures_io::AsyncSeek) for more details."] # [doc = ""] # [doc = " Note: In the edge case where you're seeking with `SeekFrom::Current(n)`"] # [doc = " where `n` minus the internal buffer length overflows an `i64`, two"] # [doc = " seeks will be performed instead of one. If the second seek returns"] # [doc = " `Err`, the underlying reader will be left at the same position it would"] # [doc = " have if you called `seek` with `SeekFrom::Current(0)`."] fn poll_seek (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { let result : u64 ; if let SeekFrom :: Current (n) = pos { let remainder = (self . cap - self . pos) as i64 ; if let Some (offset) = n . checked_sub (remainder) { result = ready ! (self . as_mut () . project () . inner . poll_seek (cx , SeekFrom :: Current (offset))) ? ; } else { ready ! (self . as_mut () . project () . inner . poll_seek (cx , SeekFrom :: Current (- remainder))) ? ; self . as_mut () . discard_buffer () ; result = ready ! (self . as_mut () . project () . inner . poll_seek (cx , SeekFrom :: Current (n))) ? ; } } else { result = ready ! (self . as_mut () . project () . inner . poll_seek (cx , pos)) ? ; } self . discard_buffer () ; Poll :: Ready (Ok (result)) } }
    };
}

impl_1067!();