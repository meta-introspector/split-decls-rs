macro_rules! deps {
    () => {
        ImageNtHeaders64!();
        PeSegment!();
    };
}

macro_rules! PeSegment64 {
    () => {
        deps!();
        # [doc = " A loadable section in a [`PeFile64`](super::PeFile64)."] pub type PeSegment64 < 'data , 'file , R = & 'data [u8] > = PeSegment < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
    };
}

PeSegment64!()