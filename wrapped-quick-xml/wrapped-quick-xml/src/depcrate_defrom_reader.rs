// Generated macro for from_reader (function)
macro_rules! Depcrate_defrom_reader {
() => {
// Module: crate::de
// Provides: {"from_reader"}
// Dependencies: {}
# [doc = " Deserialize from a reader. This method will do internal copies of data"] # [doc = " read from `reader`. If you want have a `&str` input and want to borrow"] # [doc = " as much as possible, use [`from_str`]."] pub fn from_reader < R , T > (reader : R) -> Result < T , DeError > where R : BufRead , T : DeserializeOwned , { let mut de = Deserializer :: from_reader (reader) ; T :: deserialize (& mut de) }
};
}
