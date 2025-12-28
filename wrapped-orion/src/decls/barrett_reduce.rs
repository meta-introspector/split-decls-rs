macro_rules! barrett_reduce {
    () => {
        # [doc = " Barrett reduction with correctional step."] # [doc = ""] # [doc = " Given value < 2q return value mod q (in [0, n])."] # [doc = ""] # [doc = " src: https://en.wikipedia.org/wiki/Barrett_reduction"] # [doc = " src: BoringSSL, https://boringssl.googlesource.com/boringssl/+/refs/heads/main/crypto/fipsmodule/mlkem/mlkem.cc.inc"] pub fn barrett_reduce (value : u32) -> u32 { debug_assert ! (value < KYBER_Q . pow (2)) ; const MUL : u64 = 5039 ; const SHIFT : u64 = 24 ; let quo : u32 = ((u64 :: from (value) * MUL) >> SHIFT) as u32 ; let r = value - (quo * KYBER_Q) ; debug_assert ! ((0 .. KYBER_Q * 2) . contains (& r)) ; let ret = conditional_sub_u32 (r) ; debug_assert ! ((0 .. KYBER_Q) . contains (& ret)) ; ret }
    };
}

barrett_reduce!();