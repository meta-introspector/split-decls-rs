macro_rules! deps {
    () => {
        PeSegment!();
        ImageNtHeaders32!();
    };
}

macro_rules! PeSegment32 {
    () => {
        deps!();
        # [doc = " A loadable section in a [`PeFile32`](super::PeFile32)."] pub type PeSegment32 < 'data , 'file , R = & 'data [u8] > = PeSegment < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
    };
}

PeSegment32!();