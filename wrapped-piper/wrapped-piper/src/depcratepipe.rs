// Generated macro for pipe (function)
macro_rules! Depcratepipe {
() => {
// Module: crate
// Provides: {"pipe"}
// Dependencies: {}
# [doc = " Creates a bounded single-producer single-consumer pipe."] # [doc = ""] # [doc = " A pipe is a ring buffer of `cap` bytes that can be asynchronously read from and written to."] # [doc = ""] # [doc = " See the [crate-level documentation](index.html) for more details."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function panics if `cap` is 0 or if `cap * 2` overflows a `usize`."] # [allow (clippy :: incompatible_msrv)] pub fn pipe (cap : usize) -> (Reader , Writer) { assert ! (cap > 0 , "capacity must be positive") ; assert ! (cap . checked_mul (2) . is_some () , "capacity is too large") ; let mut v = Vec :: with_capacity (cap) ; let buffer = v . as_mut_ptr () ; mem :: forget (v) ; let inner = Arc :: new (Pipe { head : AtomicUsize :: new (0) , tail : AtomicUsize :: new (0) , reader : AtomicWaker :: new () , writer : AtomicWaker :: new () , closed : AtomicBool :: new (false) , buffer , cap , }) ; let mut rng = rng () ; let r = Reader { inner : inner . clone () , head : 0 , tail : 0 , rng : rng . fork () , } ; let w = Writer { inner , head : 0 , tail : 0 , zeroed_until : 0 , rng , } ; (r , w) }
};
}
