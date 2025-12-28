macro_rules! deps {
    () => {
        TryReserveErrorKind!();
        AllocError!();
        TryReserveError!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl fmt :: Display for TryReserveError { fn fmt (& self , fmt : & mut core :: fmt :: Formatter < '_ > ,) -> core :: result :: Result < () , core :: fmt :: Error > { fmt . write_str ("memory allocation failed") ? ; let reason = match self . kind { TryReserveErrorKind :: CapacityOverflow => { " because the computed capacity exceeded the collection's maximum" } TryReserveErrorKind :: AllocError { .. } => { " because the memory allocator returned an error" } } ; fmt . write_str (reason) } }
    };
}

impl_86!()