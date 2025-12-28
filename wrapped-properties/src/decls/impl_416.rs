macro_rules! deps {
    () => {
        GeneralCategoryGroup!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl AsULE for GeneralCategoryGroup { type ULE = RawBytesULE < 2 > ; fn to_unaligned (self) -> Self :: ULE { let value = gcg_to_packed_u16 (self) ; value . to_unaligned () } fn from_unaligned (ule : Self :: ULE) -> Self { let value = ule . as_unsigned_int () ; packed_u16_to_gcg (value) } }
    };
}

impl_416!();