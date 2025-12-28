macro_rules! Bounded {
    () => {
        # [doc = " Integers whose representation takes a bounded amount of space."] pub trait Bounded { # [doc = " Size of this integer in bits."] const BITS : u32 ; # [doc = " Size of this integer in bytes."] const BYTES : usize ; }
    };
}

Bounded!();