macro_rules! LimbBuffer {
    () => {
        # [doc = " This struct represents a view of a sequence of bytes as a sequence of"] # [doc = " `u64` limbs in little-endian byte order. It maintains a current index, and"] # [doc = " allows access to the limb at that index and the one following it. Bytes"] # [doc = " beyond the end of the original buffer are treated as zero."] struct LimbBuffer < 'a > { buf : & 'a [u8] , cur_idx : usize , cur_limb : u64 , next_limb : u64 , }
    };
}

LimbBuffer!()