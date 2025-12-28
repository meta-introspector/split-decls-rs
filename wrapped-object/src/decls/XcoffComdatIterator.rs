macro_rules! deps {
    () => {
        ReadRef!();
        XcoffFile!();
        FileHeader!();
    };
}

macro_rules! XcoffComdatIterator {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffComdatIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] pub (crate) file : & 'file XcoffFile < 'data , Xcoff , R > , }
    };
}

XcoffComdatIterator!()