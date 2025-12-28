macro_rules! deps {
    () => {
        XcoffFile!();
        ReadRef!();
        FileHeader!();
    };
}

macro_rules! XcoffSegmentIterator {
    () => {
        deps!();
        # [doc = " An iterator for the segments in an [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffSegmentIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] pub (super) file : & 'file XcoffFile < 'data , Xcoff , R > , }
    };
}

XcoffSegmentIterator!();