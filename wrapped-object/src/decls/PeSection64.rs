macro_rules! deps {
    () => {
        ImageNtHeaders64!();
        PeSection!();
    };
}

macro_rules! PeSection64 {
    () => {
        deps!();
        # [doc = " A section in a [`PeFile64`](super::PeFile64)."] pub type PeSection64 < 'data , 'file , R = & 'data [u8] > = PeSection < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
    };
}

PeSection64!()