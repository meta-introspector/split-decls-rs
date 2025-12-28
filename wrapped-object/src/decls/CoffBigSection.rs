macro_rules! deps {
    () => {
        AnonObjectHeaderBigobj!();
        ObjectSection!();
        CoffSection!();
    };
}

macro_rules! CoffBigSection {
    () => {
        deps!();
        # [doc = " A section in a [`CoffBigFile`](super::CoffBigFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSection`] trait implementation."] pub type CoffBigSection < 'data , 'file , R = & 'data [u8] > = CoffSection < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigSection!()