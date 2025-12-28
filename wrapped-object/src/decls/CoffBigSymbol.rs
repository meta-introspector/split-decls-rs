macro_rules! deps {
    () => {
        CoffSymbol!();
        AnonObjectHeaderBigobj!();
        ObjectSymbol!();
    };
}

macro_rules! CoffBigSymbol {
    () => {
        deps!();
        # [doc = " A symbol in a [`CoffBigFile`](super::CoffBigFile)."] # [doc = ""] # [doc = " Most functionality is provided by the [`ObjectSymbol`] trait implementation."] pub type CoffBigSymbol < 'data , 'file , R = & 'data [u8] > = CoffSymbol < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigSymbol!()