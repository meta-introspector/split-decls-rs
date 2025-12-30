// Generated macro for AsStaticStr (struct)
macro_rules! Depcrate_converterAsStaticStr {
() => {
// Module: crate::converter
// Provides: {"AsStaticStr"}
// Dependencies: {}
# [doc = " Let [`Bake`] output a `&'static str` for anything that can be `AsRef<str>`."] # [doc = ""] # [doc = " It can be helpful if the type needs to be constructed (e.g. [`String`], [`Cow<str>`](std::borrow::Cow)) but should result in a `&'static str`."] # [doc = ""] # [doc = " This requires that the crate using the generated data needs a different struct."] # [doc = ""] # [doc = " ```"] # [doc = " use databake::{converter::AsStaticStr, Bake};"] # [doc = ""] # [doc = " #[derive(Bake)]"] # [doc = " #[databake(path = my_crate)]"] # [doc = " struct Data {"] # [doc = "     pub number: usize,"] # [doc = "     pub string: AsStaticStr<String>, // can be written as StringAsStaticStr"] # [doc = " }"] # [doc = ""] # [doc = " let data = Data {"] # [doc = "     number: 6,"] # [doc = "     string: 6.to_string().into(),"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     data.bake(&Default::default()).to_string(),"] # [doc = "     r#\"my_crate :: Data { number : 6usize , string : \"6\" , }\"#"] # [doc = " );"] # [doc = ""] # [doc = " mod my_crate {"] # [doc = "     struct Data {"] # [doc = "         number: usize,"] # [doc = "         string: &'static str,"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (Default)] # [repr (transparent)] pub struct AsStaticStr < T > (pub T) where T : AsRef < str > ;
};
}
