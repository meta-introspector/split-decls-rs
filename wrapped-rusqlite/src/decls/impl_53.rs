macro_rules! deps {
    () => {
        Batch!();
        Result!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'conn > fallible_iterator :: FallibleIterator for Batch < 'conn , '_ > { type Error = Error ; type Item = Statement < 'conn > ; # [doc = " Iterates on each batch statements."] # [doc = ""] # [doc = " Returns `Ok(None)` when batch is completed."] fn next (& mut self) -> Result < Option < Statement < 'conn > > > { while self . tail < self . sql . len () { let sql = & self . sql [self . tail ..] ; let (next , tail) = self . conn . db . borrow_mut () . prepare (self . conn , sql , PrepFlags :: default ()) ? ; if tail == 0 { self . tail = self . sql . len () ; } else { self . tail += tail ; } if next . stmt . is_null () { continue ; } return Ok (Some (next)) ; } Ok (None) } }
    };
}

impl_53!()