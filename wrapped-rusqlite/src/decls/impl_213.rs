macro_rules! deps {
    () => {
        Error!();
        Rows!();
        Row!();
        Statement!();
        Result!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < 'stmt > Rows < 'stmt > { # [inline] pub (crate) fn new (stmt : & 'stmt Statement < 'stmt >) -> Self { Rows { stmt : Some (stmt) , row : None , } } # [inline] pub (crate) fn get_expected_row (& mut self) -> Result < & Row < 'stmt > > { match self . next () ? { Some (row) => Ok (row) , None => Err (Error :: QueryReturnedNoRows) , } } }
    };
}

impl_213!();