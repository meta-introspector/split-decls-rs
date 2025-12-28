macro_rules! deps {
    () => {
        XcoffFile!();
        FileHeader!();
        ReadRef!();
    };
}

macro_rules! XcoffSegment {
    () => {
        deps!();
        # [doc = " A loadable section in an [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffSegment < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , }
    };
}

XcoffSegment!();