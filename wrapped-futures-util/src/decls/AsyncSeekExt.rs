macro_rules! deps {
    () => {
        Seek!();
        Current!();
    };
}

macro_rules! AsyncSeekExt {
    () => {
        deps!();
        # [doc = " An extension trait which adds utility methods to `AsyncSeek` types."] pub trait AsyncSeekExt : AsyncSeek { # [doc = " Creates a future which will seek an IO object, and then yield the"] # [doc = " new position in the object and the object itself."] # [doc = ""] # [doc = " In the case of an error the buffer and the object will be discarded, with"] # [doc = " the error yielded."] fn seek (& mut self , pos : SeekFrom) -> Seek < '_ , Self > where Self : Unpin , { assert_future :: < Result < u64 > , _ > (Seek :: new (self , pos)) } # [doc = " Creates a future which will return the current seek position from the"] # [doc = " start of the stream."] # [doc = ""] # [doc = " This is equivalent to `self.seek(SeekFrom::Current(0))`."] fn stream_position (& mut self) -> Seek < '_ , Self > where Self : Unpin , { self . seek (SeekFrom :: Current (0)) } }
    };
}

AsyncSeekExt!();