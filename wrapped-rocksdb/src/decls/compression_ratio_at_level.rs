macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! compression_ratio_at_level {
    () => {
        deps!();
        # [doc = " \"rocksdb.compression-ratio-at-level<`N`>\" - returns string containing the"] # [doc = " compression ratio of data at level <`N`>, where <`N`> is an ASCII"] # [doc = " representation of a level number (e.g., \"0\"). Here, compression"] # [doc = " ratio is defined as uncompressed data size / compressed file size."] # [doc = " Returns \"-1.0\" if no open files at level <`N`>."] pub fn compression_ratio_at_level (level : usize) -> PropertyName { unsafe { level_property ("compression-ratio-at-level" , level) } }
    };
}

compression_ratio_at_level!();