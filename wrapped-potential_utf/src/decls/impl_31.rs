macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `zerovec` Cargo feature"] # [cfg (all (feature = "zerovec" , feature = "alloc"))] impl < 'a > zerovec :: maps :: ZeroMapKV < 'a > for PotentialUtf8 { type Container = zerovec :: VarZeroVec < 'a , PotentialUtf8 > ; type Slice = zerovec :: VarZeroSlice < PotentialUtf8 > ; type GetType = PotentialUtf8 ; type OwnedType = Box < PotentialUtf8 > ; }
    };
}

impl_31!()