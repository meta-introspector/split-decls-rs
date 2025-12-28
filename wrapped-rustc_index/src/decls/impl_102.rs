macro_rules! deps {
    () => {
        IndexSlice!();
        IndexVec!();
        Idx!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < I : Idx , T : Clone > ToOwned for IndexSlice < I , T > { type Owned = IndexVec < I , T > ; fn to_owned (& self) -> IndexVec < I , T > { IndexVec :: from_raw (self . raw . to_owned ()) } fn clone_into (& self , target : & mut IndexVec < I , T >) { self . raw . clone_into (& mut target . raw) } }
    };
}

impl_102!();