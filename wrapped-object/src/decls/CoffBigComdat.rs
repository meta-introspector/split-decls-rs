macro_rules! deps {
    () => {
        AnonObjectHeaderBigobj!();
        ObjectComdat!();
        CoffComdat!();
    };
}

macro_rules! CoffBigComdat {
    () => {
        deps!();
        # [doc = " A COMDAT section group in a [`CoffBigFile`](super::CoffBigFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectComdat`] trait implementation."] pub type CoffBigComdat < 'data , 'file , R = & 'data [u8] > = CoffComdat < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigComdat!()