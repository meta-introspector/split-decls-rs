macro_rules! Block {
    () => {
        pub unsafe trait Block { type Item ; fn capacity () -> usize ; }
    };
}

Block!();