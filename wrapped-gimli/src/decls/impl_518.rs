macro_rules! deps {
    () => {
        Expression!();
        OperationIter!();
        Reader!();
        Result!();
        Operation!();
    };
}

macro_rules! impl_518 {
    () => {
        deps!();
        impl < R : Reader > OperationIter < R > { # [doc = " Read the next operation in an expression."] pub fn next (& mut self) -> Result < Option < Operation < R > > > { if self . input . is_empty () { return Ok (None) ; } match Operation :: parse (& mut self . input , self . encoding) { Ok (op) => Ok (Some (op)) , Err (e) => { self . input . empty () ; Err (e) } } } # [doc = " Return the current byte offset of the iterator."] pub fn offset_from (& self , expression : & Expression < R >) -> R :: Offset { self . input . offset_from (& expression . 0) } }
    };
}

impl_518!();