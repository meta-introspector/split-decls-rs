macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < D : Decoder , I : Idx , T : Decodable < D > > Decodable < D > for IndexVec < I , T > { fn decode (d : & mut D) -> Self { IndexVec :: from_raw (Vec :: < T > :: decode (d)) } }
    };
}

impl_123!()