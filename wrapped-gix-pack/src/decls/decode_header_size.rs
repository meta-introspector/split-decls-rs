macro_rules! decode_header_size {
    () => {
        # [doc = " Given the decompressed pack delta `d`, decode a size in bytes (either the base object size or the result object size)"] # [doc = " Equivalent to [this canonical git function](https://github.com/git/git/blob/311531c9de557d25ac087c1637818bd2aad6eb3a/delta.h#L89)"] pub (crate) fn decode_header_size (d : & [u8]) -> (u64 , usize) { let mut i = 0 ; let mut size = 0u64 ; let mut consumed = 0 ; for cmd in d . iter () { consumed += 1 ; size |= (u64 :: from (* cmd) & 0x7f) << i ; i += 7 ; if * cmd & 0x80 == 0 { break ; } } (size , consumed) }
    };
}

decode_header_size!();