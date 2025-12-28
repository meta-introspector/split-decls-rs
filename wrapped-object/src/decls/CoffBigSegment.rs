macro_rules! deps {
    () => {
        ObjectSegment!();
        CoffSegment!();
        AnonObjectHeaderBigobj!();
    };
}

macro_rules! CoffBigSegment {
    () => {
        deps!();
        # [doc = " A loadable section in a [`CoffBigFile`](super::CoffBigFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSegment`] trait implementation."] pub type CoffBigSegment < 'data , 'file , R = & 'data [u8] > = CoffSegment < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigSegment!()