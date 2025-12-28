macro_rules! deps {
    () => {
        FileHeader!();
        XcoffFile!();
        ReadRef!();
    };
}

macro_rules! XcoffComdatSectionIterator {
    () => {
        deps!();
        # [doc = " An iterator for the sections in a COMDAT section group in a [`XcoffFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct XcoffComdatSectionIterator < 'data , 'file , Xcoff , R = & 'data [u8] > where Xcoff : FileHeader , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file XcoffFile < 'data , Xcoff , R > , }
    };
}

XcoffComdatSectionIterator!();