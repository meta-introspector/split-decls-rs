macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T , N : ArrayLength + ArraySize > GenericArray < T , N > { # [doc = " From `&self` of this version, create a reference to a [`Array`](hybrid_array_0_4::Array) from `hybrid-array` 0.4."] # [inline (always)] pub const fn as_ha0_4 (& self) -> & HybridArray < T , N > { unsafe { core :: mem :: transmute (self) } } # [doc = " From `&mut self` of this version, create a mutable reference to a [`Array`](hybrid_array_0_4::Array) from `hybrid-array` 0.4."] # [doc = ""] # [doc = " This method is `const` since Rust 1.83.0, but non-`const` before."] # [rustversion :: attr (since (1.83) , const)] # [inline (always)] pub fn as_ha0_4_mut (& mut self) -> & mut HybridArray < T , N > { unsafe { core :: mem :: transmute (self) } } # [doc = " From `self` of this version, create a [`Array`](hybrid_array_0_4::Array) from `hybrid-array` 0.4."] # [inline (always)] pub const fn into_ha0_4 (self) -> HybridArray < T , N > { unsafe { crate :: const_transmute (self) } } # [doc = " From a [`Array`](hybrid_array_0_4::Array) from `hybrid-array` 0.4, create a [`GenericArray`] of this version."] # [inline (always)] pub const fn from_ha0_4 (value : HybridArray < T , N >) -> Self { unsafe { crate :: const_transmute (value) } } }
    };
}

impl_15!();