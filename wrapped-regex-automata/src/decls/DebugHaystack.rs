macro_rules! DebugHaystack {
    () => {
        # [doc = " Provides a convenient `Debug` implementation for `&[u8]`."] # [doc = ""] # [doc = " This generally works best when the bytes are presumed to be mostly UTF-8,"] # [doc = " but will work for anything. For any bytes that aren't UTF-8, they are"] # [doc = " emitted as hex escape sequences."] pub struct DebugHaystack < 'a > (pub & 'a [u8]) ;
    };
}

DebugHaystack!();