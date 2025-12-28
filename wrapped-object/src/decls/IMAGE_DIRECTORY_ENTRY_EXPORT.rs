macro_rules! deps {
    () => {
        Export!();
    };
}

macro_rules! IMAGE_DIRECTORY_ENTRY_EXPORT {
    () => {
        deps!();
        # [doc = " Export Directory"] pub const IMAGE_DIRECTORY_ENTRY_EXPORT : usize = 0 ;
    };
}

IMAGE_DIRECTORY_ENTRY_EXPORT!()