macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < S : Encoder , I : Idx , T : Encodable < S > > Encodable < S > for IndexVec < I , T > { fn encode (& self , s : & mut S) { Encodable :: encode (& self . raw , s) ; } }
    };
}

impl_122!();