macro_rules! Iter {
    () => {
        # [doc = " An iterator over the data of index extensions."] pub struct Iter < 'a > { data : & 'a [u8] , # [doc = " The amount of consumed bytes as seen from our internal data pointer. Useful to continue where the iterator left off."] pub consumed : usize , }
    };
}

Iter!()