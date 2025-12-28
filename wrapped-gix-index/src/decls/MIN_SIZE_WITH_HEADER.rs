macro_rules! MIN_SIZE_WITH_HEADER {
    () => {
        # [doc = " The smallest size of the extension varying by hash kind, along with the standard extension header."] pub const MIN_SIZE_WITH_HEADER : usize = extension :: MIN_SIZE + MIN_SIZE ;
    };
}

MIN_SIZE_WITH_HEADER!()