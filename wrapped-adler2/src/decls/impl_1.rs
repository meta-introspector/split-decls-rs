macro_rules! deps {
    () => {
        Adler32!();
        U32X4!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Adler32 { pub (crate) fn compute (& mut self , bytes : & [u8]) { const MOD : u32 = 65521 ; const CHUNK_SIZE : usize = 5552 * 4 ; let mut a = u32 :: from (self . a) ; let mut b = u32 :: from (self . b) ; let mut a_vec = U32X4 ([0 ; 4]) ; let mut b_vec = a_vec ; let (bytes , remainder) = bytes . split_at (bytes . len () - bytes . len () % 4) ; let chunk_iter = bytes . chunks_exact (CHUNK_SIZE) ; let remainder_chunk = chunk_iter . remainder () ; for chunk in chunk_iter { for byte_vec in chunk . chunks_exact (4) { let val = U32X4 :: from (byte_vec) ; a_vec += val ; b_vec += a_vec ; } b += CHUNK_SIZE as u32 * a ; a_vec %= MOD ; b_vec %= MOD ; b %= MOD ; } for byte_vec in remainder_chunk . chunks_exact (4) { let val = U32X4 :: from (byte_vec) ; a_vec += val ; b_vec += a_vec ; } b += remainder_chunk . len () as u32 * a ; a_vec %= MOD ; b_vec %= MOD ; b %= MOD ; b_vec *= 4 ; b_vec . 0 [1] += MOD - a_vec . 0 [1] ; b_vec . 0 [2] += (MOD - a_vec . 0 [2]) * 2 ; b_vec . 0 [3] += (MOD - a_vec . 0 [3]) * 3 ; for & av in a_vec . 0 . iter () { a += av ; } for & bv in b_vec . 0 . iter () { b += bv ; } for & byte in remainder . iter () { a += u32 :: from (byte) ; b += a ; } self . a = (a % MOD) as u16 ; self . b = (b % MOD) as u16 ; } }
    };
}

impl_1!();