macro_rules! deps {
    () => {
        ImageNtHeaders!();
        PeFile!();
        ReadRef!();
    };
}

macro_rules! PeComdatIterator {
    () => {
        deps!();
        # [doc = " An iterator for the COMDAT section groups in a [`PeFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct PeComdatIterator < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file PeFile < 'data , Pe , R > , }
    };
}

PeComdatIterator!()