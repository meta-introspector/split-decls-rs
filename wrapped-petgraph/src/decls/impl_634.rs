macro_rules! deps {
    () => {
        EdgeType!();
        EdgeProperty!();
        FromDeserialized!();
    };
}

macro_rules! impl_634 {
    () => {
        deps!();
        impl < Ty > FromDeserialized for PhantomData < Ty > where Ty : EdgeType , { type Input = EdgeProperty ; fn from_deserialized < E2 > (input : Self :: Input) -> Result < Self , E2 > where E2 : Error , { if input . is_directed () != Ty :: is_directed () { Err (E2 :: custom (format_args ! ("graph edge property mismatch, \
                 expected {:?}, found {:?}" , EdgeProperty :: from (PhantomData ::< Ty >) , input))) } else { Ok (PhantomData) } } }
    };
}

impl_634!();