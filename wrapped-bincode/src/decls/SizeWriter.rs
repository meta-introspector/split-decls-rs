macro_rules! SizeWriter {
    () => {
        # [doc = " A writer that counts how many bytes were written. This is useful for e.g. pre-allocating buffers before writing to them."] # [derive (Default)] pub struct SizeWriter { # [doc = " the amount of bytes that were written so far"] pub bytes_written : usize , }
    };
}

SizeWriter!();