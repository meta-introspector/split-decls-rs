macro_rules! deps {
    () => {
        Group!();
        GroupEncoding!();
    };
}

macro_rules! PrimeGroup {
    () => {
        deps!();
        # [doc = " This trait represents an element of a prime-order cryptographic group."] pub trait PrimeGroup : Group + GroupEncoding { }
    };
}

PrimeGroup!()