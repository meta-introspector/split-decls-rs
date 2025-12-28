macro_rules! deps {
    () => {
        UnescapeState!();
        UnescapeBytes!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I : Iterator < Item = char > > UnescapeBytes < I > { pub (crate) fn new < T : IntoIterator < IntoIter = I > > (t : T ,) -> UnescapeBytes < I > { UnescapeBytes { it : t . into_iter () , state : UnescapeState :: Start } } }
    };
}

impl_51!();