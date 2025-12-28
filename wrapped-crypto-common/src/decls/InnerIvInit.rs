macro_rules! deps {
    () => {
        IvSizeUser!();
        InnerUser!();
        InvalidLength!();
        Iv!();
    };
}

macro_rules! InnerIvInit {
    () => {
        deps!();
        # [doc = " Types which can be initialized from another type and additional initialization"] # [doc = " vector/nonce."] # [doc = ""] # [doc = " Usually used for initializing types from block ciphers."] pub trait InnerIvInit : InnerUser + IvSizeUser + Sized { # [doc = " Initialize value using `inner` and `iv` array."] fn inner_iv_init (inner : Self :: Inner , iv : & Iv < Self >) -> Self ; # [doc = " Initialize value using `inner` and `iv` slice."] # [inline] fn inner_iv_slice_init (inner : Self :: Inner , iv : & [u8]) -> Result < Self , InvalidLength > { let iv = < & Iv < Self > > :: try_from (iv) . map_err (| _ | InvalidLength) ? ; Ok (Self :: inner_iv_init (inner , iv)) } # [doc = " Generate random IV using the operating system's secure RNG."] # [cfg (feature = "getrandom")] # [inline] fn generate_iv () -> Result < Iv < Self > , getrandom :: Error > { let mut iv = Iv :: < Self > :: default () ; getrandom :: fill (& mut iv) ? ; Ok (iv) } # [doc = " Generate random IV using the provided [`CryptoRng`]."] # [cfg (feature = "rand_core")] # [inline] fn generate_iv_with_rng < R : CryptoRng + ? Sized > (rng : & mut R) -> Iv < Self > { let mut iv = Iv :: < Self > :: default () ; rng . fill_bytes (& mut iv) ; iv } # [doc = " Generate random IV using the provided [`TryCryptoRng`]."] # [cfg (feature = "rand_core")] # [inline] fn try_generate_iv_with_rng < R : TryCryptoRng + ? Sized > (rng : & mut R ,) -> Result < Iv < Self > , R :: Error > { let mut iv = Iv :: < Self > :: default () ; rng . try_fill_bytes (& mut iv) ? ; Ok (iv) } }
    };
}

InnerIvInit!()