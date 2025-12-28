macro_rules! deps {
    () => {
        PeFile!();
        ImageNtHeaders!();
        ReadRef!();
    };
}

macro_rules! PeComdat {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`PeFile`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct PeComdat < 'data , 'file , Pe , R = & 'data [u8] > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { # [allow (unused)] file : & 'file PeFile < 'data , Pe , R > , }
    };
}

PeComdat!()