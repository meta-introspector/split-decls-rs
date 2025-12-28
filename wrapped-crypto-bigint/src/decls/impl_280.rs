macro_rules! deps {
    () => {
        RandomBitsError!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        # [cfg (feature = "rand_core")] impl < T > fmt :: Display for RandomBitsError < T > where T : fmt :: Display , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: RandCore (err) => write ! (f , "{err}") , Self :: BitsPrecisionMismatch { bits_precision , integer_bits , } => write ! (f , concat ! ["The requested `bits_precision` ({}) does not match " , "the size of the integer corresponding to the type ({})"] , bits_precision , integer_bits) , Self :: BitLengthTooLarge { bit_length , bits_precision , } => write ! (f , "The requested `bit_length` ({bit_length}) is larger than `bits_precision` ({bits_precision})." ,) , } } }
    };
}

impl_280!()