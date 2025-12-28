macro_rules! deps {
    () => {
        ImageDataDirectory!();
    };
}

macro_rules! DataDirectories {
    () => {
        deps!();
        # [doc = " The table of data directories in a PE file."] # [doc = ""] # [doc = " Returned by [`ImageNtHeaders::parse`](super::ImageNtHeaders::parse)."] # [derive (Debug , Clone , Copy)] pub struct DataDirectories < 'data > { entries : & 'data [pe :: ImageDataDirectory] , }
    };
}

DataDirectories!();