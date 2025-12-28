macro_rules! deps {
    () => {
        GenericSequence!();
        Mapped!();
    };
}

macro_rules! MappedGenericSequence {
    () => {
        deps!();
        # [doc = " Defines the relationship between one generic sequence and another,"] # [doc = " for operations such as `map` and `zip`."] pub trait MappedGenericSequence < T , U > : GenericSequence < T > { # [doc = " Mapped sequence type"] type Mapped : GenericSequence < U , Length = Self :: Length > ; }
    };
}

MappedGenericSequence!()