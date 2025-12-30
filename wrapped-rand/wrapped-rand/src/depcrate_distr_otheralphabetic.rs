// Generated macro for Alphabetic (struct)
macro_rules! Depcrate_distr_otherAlphabetic {
() => {
// Module: crate::distr::other
// Provides: {"Alphabetic"}
// Dependencies: {}
# [doc = " Sample a [`u8`], uniformly distributed over letters:"] # [doc = " a-z and A-Z."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " You're able to generate random Alphabetic characters via mapping or via the"] # [doc = " [`SampleString::sample_string`] method like so:"] # [doc = ""] # [doc = " ```"] # [doc = " use rand::Rng;"] # [doc = " use rand::distr::{Alphabetic, SampleString};"] # [doc = ""] # [doc = " // Manual mapping"] # [doc = " let mut rng = rand::rng();"] # [doc = " let chars: String = (0..7).map(|_| rng.sample(Alphabetic) as char).collect();"] # [doc = " println!(\"Random chars: {}\", chars);"] # [doc = ""] # [doc = " // Using [`SampleString::sample_string`]"] # [doc = " let string = Alphabetic.sample_string(&mut rand::rng(), 16);"] # [doc = " println!(\"Random string: {}\", string);"] # [doc = " ```"] # [doc = ""] # [doc = " # Passwords"] # [doc = ""] # [doc = " Refer to [`Alphanumeric#Passwords`]."] # [derive (Debug , Clone , Copy , Default)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Alphabetic ;
};
}
