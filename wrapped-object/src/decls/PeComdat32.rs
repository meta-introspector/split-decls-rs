macro_rules! deps {
    () => {
        ImageNtHeaders32!();
        PeFile32!();
        PeComdat!();
    };
}

macro_rules! PeComdat32 {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`PeFile32`]."] pub type PeComdat32 < 'data , 'file , R = & 'data [u8] > = PeComdat < 'data , 'file , pe :: ImageNtHeaders32 , R > ;
    };
}

PeComdat32!()