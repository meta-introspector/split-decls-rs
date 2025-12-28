macro_rules! HeapString {
    () => {
        pub struct HeapString (pub * mut u16) ;
    };
}

HeapString!();