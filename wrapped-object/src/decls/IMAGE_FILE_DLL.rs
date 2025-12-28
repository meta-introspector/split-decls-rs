macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! IMAGE_FILE_DLL {
    () => {
        deps!();
        # [doc = " File is a DLL."] pub const IMAGE_FILE_DLL : u16 = 0x2000 ;
    };
}

IMAGE_FILE_DLL!();