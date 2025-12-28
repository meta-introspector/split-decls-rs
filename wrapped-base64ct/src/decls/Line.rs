macro_rules! Line {
    () => {
        # [doc = " A single line of linewrapped data, providing a read buffer."] # [derive (Clone , Debug)] pub struct Line < 'i > { # [doc = " Remaining data in the line"] remaining : & 'i [u8] , }
    };
}

Line!();