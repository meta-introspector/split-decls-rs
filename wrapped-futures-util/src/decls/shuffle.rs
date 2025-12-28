macro_rules! shuffle {
    () => {
        # [doc (hidden)] pub fn shuffle < T > (slice : & mut [T]) { for i in (1 .. slice . len ()) . rev () { slice . swap (i , gen_index (i + 1)) ; } }
    };
}

shuffle!()