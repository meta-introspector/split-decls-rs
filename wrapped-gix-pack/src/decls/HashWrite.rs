macro_rules! HashWrite {
    () => {
        # [doc = " A utility to automatically generate a hash while writing into an inner writer."] pub struct HashWrite < 'a , T > { # [doc = " The hash implementation."] pub hash : & 'a mut Hasher , # [doc = " The inner writer."] pub inner : T , }
    };
}

HashWrite!();