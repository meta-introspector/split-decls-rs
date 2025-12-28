macro_rules! deps {
    () => {
        Scalar!();
        FieldElement!();
    };
}

macro_rules! mont_ladder {
    () => {
        deps!();
        # [doc = " Scalar multiplication using the Montgomery Ladder (a.k.a \"scalarmult\")"] # [doc = ""] # [doc = " Refs:"] # [doc = " - https://eprint.iacr.org/2020/956.pdf"] # [doc = " - https://eprint.iacr.org/2017/212.pdf"] # [doc = " - https://github.com/golang/crypto/blob/0c34fe9e7dc2486962ef9867e3edb3503537209f/curve25519/curve25519_generic.go#L779"] fn mont_ladder (scalar : & Scalar , point : FieldElement) -> FieldElement { let x1 = point ; let mut x2 = FieldElement :: one () ; let mut x3 = x1 ; let mut z3 = FieldElement :: one () ; let mut z2 = FieldElement :: zero () ; let mut tmp0 : FieldElement ; let mut tmp1 : FieldElement ; let mut swap : u8 = 0 ; for idx in (0 ..= 254) . rev () { let bit = (scalar . 0 [idx >> 3] >> (idx & 7)) & 1 ; swap ^= bit ; FieldElement :: conditional_swap (swap , & mut x2 , & mut x3) ; FieldElement :: conditional_swap (swap , & mut z2 , & mut z3) ; swap = bit ; tmp0 = x3 - z3 ; tmp1 = x2 - z2 ; x2 = x2 + z2 ; z2 = x3 + z3 ; z3 = tmp0 * x2 ; z2 = z2 * tmp1 ; tmp0 = tmp1 . square () ; tmp1 = x2 . square () ; x3 = z3 + z2 ; z2 = z3 - z2 ; x2 = tmp1 * tmp0 ; tmp1 = tmp1 - tmp0 ; z2 = z2 . square () ; z3 = tmp1 . mul_121666 () ; x3 = x3 . square () ; tmp0 = tmp0 + z3 ; z3 = x1 * z2 ; z2 = tmp1 * tmp0 ; } FieldElement :: conditional_swap (swap , & mut x2 , & mut x3) ; FieldElement :: conditional_swap (swap , & mut z2 , & mut z3) ; z2 . invert () ; x2 = x2 * z2 ; x2 }
    };
}

mont_ladder!()