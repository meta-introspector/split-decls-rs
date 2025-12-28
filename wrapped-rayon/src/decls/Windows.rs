macro_rules! Windows {
    () => {
        # [doc = " Parallel iterator over immutable overlapping windows of a slice"] # [derive (Debug)] pub struct Windows < 'data , T > { window_size : usize , slice : & 'data [T] , }
    };
}

Windows!();