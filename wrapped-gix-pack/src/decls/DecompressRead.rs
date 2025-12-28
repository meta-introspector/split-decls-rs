macro_rules! DecompressRead {
    () => {
        # [doc = " The boxed variant is faster for what we do (moving the decompressor in and out a lot)"] pub struct DecompressRead < 'a , R > { # [doc = " The reader from which bytes should be decompressed."] pub inner : R , # [doc = " The decompressor doing all the work."] pub decompressor : & 'a mut Decompress , }
    };
}

DecompressRead!()