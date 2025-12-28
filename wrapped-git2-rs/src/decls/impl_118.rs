macro_rules! deps {
    () => {
        Oid!();
        OidArray!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl Deref for OidArray { type Target = [Oid] ; fn deref (& self) -> & [Oid] { unsafe { debug_assert_eq ! (mem :: size_of ::< Oid > () , mem :: size_of_val (&* self . raw . ids)) ; slice :: from_raw_parts (self . raw . ids as * const Oid , self . raw . count as usize) } } }
    };
}

impl_118!()