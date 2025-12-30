// Generated macro for StrWrite (trait)
macro_rules! DepcrateStrWrite {
() => {
// Module: crate
// Provides: {"StrWrite"}
// Dependencies: {}
# [doc = " Trait that allows writing string slices. This is basically an extension"] # [doc = " of `std::io::Write` in order to include `String`."] pub trait StrWrite { type Error ; fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > ; fn write_fmt (& mut self , args : Arguments) -> Result < () , Self :: Error > ; }
};
}
