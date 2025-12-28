macro_rules! deps {
    () => {
        PeComdat!();
        ImageNtHeaders64!();
        PeFile64!();
    };
}

macro_rules! PeComdat64 {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`PeFile64`]."] pub type PeComdat64 < 'data , 'file , R = & 'data [u8] > = PeComdat < 'data , 'file , pe :: ImageNtHeaders64 , R > ;
    };
}

PeComdat64!()