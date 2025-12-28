macro_rules! deps {
    () => {
        AnonObjectHeaderBigobj!();
        CoffSymbolTable!();
    };
}

macro_rules! CoffBigSymbolTable {
    () => {
        deps!();
        # [doc = " A symbol table in a [`CoffBigFile`](super::CoffBigFile)."] pub type CoffBigSymbolTable < 'data , 'file , R = & 'data [u8] > = CoffSymbolTable < 'data , 'file , R , pe :: AnonObjectHeaderBigobj > ;
    };
}

CoffBigSymbolTable!()