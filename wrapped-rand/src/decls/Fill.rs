macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! Fill {
    () => {
        deps!();
        # [doc = " Support filling a slice with random data"] # [doc = ""] # [doc = " This trait allows slices of \"plain data\" types to be efficiently filled"] # [doc = " with random data."] # [doc = ""] # [doc = " Implementations are expected to be portable across machines unless"] # [doc = " clearly documented otherwise (see the"] # [doc = " [Chapter on Portability](https://rust-random.github.io/book/portability.html))."] # [doc = " The implementations provided achieve this by byte-swapping on big-endian"] # [doc = " machines."] pub trait Fill : Sized { # [doc = " Fill this with random data"] fn fill_slice < R : Rng + ? Sized > (this : & mut [Self] , rng : & mut R) ; }
    };
}

Fill!();