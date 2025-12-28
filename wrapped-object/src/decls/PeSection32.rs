macro_rules! deps {
    () => {
        PeSection!();
        ImageNtHeaders32!();
    };
}

macro_rules! PeSection32 {
    () => {
        deps!();
        # [doc = " A section in a [`PeFile32`](super::PeFile32)."] pub type PeSection32 < 'data , 'file , R = & 'data [u8] > = PeSection < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
    };
}

PeSection32!();