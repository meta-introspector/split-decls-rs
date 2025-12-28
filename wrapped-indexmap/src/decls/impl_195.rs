macro_rules! deps {
    () => {
        TryReserveErrorKind!();
        TryReserveError!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl core :: fmt :: Display for TryReserveError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let reason = match & self . kind { TryReserveErrorKind :: Std (e) => return core :: fmt :: Display :: fmt (e , f) , TryReserveErrorKind :: CapacityOverflow => { " because the computed capacity exceeded the collection's maximum" } TryReserveErrorKind :: AllocError { .. } => { " because the memory allocator returned an error" } } ; f . write_str ("memory allocation failed") ? ; f . write_str (reason) } }
    };
}

impl_195!()