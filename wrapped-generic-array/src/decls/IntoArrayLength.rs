macro_rules! deps {
    () => {
        ArrayLength!();
        ConstArrayLength!();
    };
}

macro_rules! IntoArrayLength {
    () => {
        deps!();
        # [doc = " Implemented for types which can have an associated [`ArrayLength`],"] # [doc = " such as [`Const<N>`] for use with const-generics."] # [doc = ""] # [doc = " ```"] # [doc = " use generic_array::{GenericArray, IntoArrayLength, ConstArrayLength, typenum::Const};"] # [doc = ""] # [doc = " fn some_array_interopt<const N: usize>(value: [u32; N]) -> GenericArray<u32, ConstArrayLength<N>>"] # [doc = " where"] # [doc = "     Const<N>: IntoArrayLength,"] # [doc = " {"] # [doc = "     let ga = GenericArray::from(value);"] # [doc = "     // do stuff"] # [doc = "     ga"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This is mostly to simplify the `where` bounds, equivalent to:"] # [doc = ""] # [doc = " ```"] # [doc = " use generic_array::{GenericArray, ArrayLength, typenum::{Const, U, ToUInt}};"] # [doc = ""] # [doc = " fn some_array_interopt<const N: usize>(value: [u32; N]) -> GenericArray<u32, U<N>>"] # [doc = " where"] # [doc = "     Const<N>: ToUInt,"] # [doc = "     U<N>: ArrayLength,"] # [doc = " {"] # [doc = "     let ga = GenericArray::from(value);"] # [doc = "     // do stuff"] # [doc = "     ga"] # [doc = " }"] # [doc = " ```"] pub trait IntoArrayLength { # [doc = " The associated `ArrayLength`"] type ArrayLength : ArrayLength ; }
    };
}

IntoArrayLength!()