macro_rules! deps {
    () => {
        FileHeader!();
        XcoffFile!();
        ReadRef!();
    };
}

macro_rules! XcoffComdat {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffComdat < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file XcoffFile < 'data , Xcoff , R > , }
    };
}

XcoffComdat!();