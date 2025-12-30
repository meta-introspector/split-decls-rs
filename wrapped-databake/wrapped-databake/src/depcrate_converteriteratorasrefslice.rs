// Generated macro for IteratorAsRefSlice (struct)
macro_rules! Depcrate_converterIteratorAsRefSlice {
() => {
// Module: crate::converter
// Provides: {"IteratorAsRefSlice"}
// Dependencies: {}
# [doc = " Let [`Bake`] output a `&'static [T]` for anything that can be iterated over."] # [doc = ""] # [doc = " It can be helpful if the type needs to be constructed (e.g. [`Vec<T>`]) but should result in a `&'static [T]`."] # [doc = ""] # [doc = " This requires that the crate using the generated data needs a different struct."] # [doc = ""] # [doc = " ```"] # [doc = " use databake::{converter::IteratorAsRefSlice, Bake};"] # [doc = ""] # [doc = " #[derive(Bake, Default)]"] # [doc = " #[databake(path = my_crate)]"] # [doc = " struct Data {"] # [doc = "     pub numbers: IteratorAsRefSlice<Vec<usize>, usize>, // can be written as `VecAsRefSlice<usize>`"] # [doc = " }"] # [doc = ""] # [doc = " let mut data = Data::default();"] # [doc = " data.numbers.push(6);"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     data.bake(&Default::default()).to_string(),"] # [doc = "     r#\"my_crate :: Data { numbers : & [6usize ,] , }\"#"] # [doc = " );"] # [doc = ""] # [doc = " mod my_crate {"] # [doc = "     struct Data {"] # [doc = "         numbers: &'static [usize],"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (Default)] # [repr (transparent)] pub struct IteratorAsRefSlice < B , T > (pub B , pub PhantomData < T >) where for < 'a > & 'a B : IntoIterator < Item = & 'a T > , T : Bake ;
};
}
