macro_rules! deps {
    () => {
        CoffSymbolIterator!();
        AnonObjectHeaderBigobj!();
    };
}

macro_rules! CoffBigSymbolIterator {
    () => {
        deps!();
        # [doc = " An iterator for the symbols in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigSymbolIterator < 'data , 'file , R = & 'data [u8] > = CoffSymbolIterator < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigSymbolIterator!();