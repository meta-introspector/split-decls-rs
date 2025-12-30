// Generated macro for Suffix (struct)
macro_rules! Depcrate_arch_all_twowaySuffix {
() => {
// Module: crate::arch::all::twoway
// Provides: {"Suffix"}
// Dependencies: {}
# [doc = " A suffix extracted from a needle along with its period."] # [derive (Debug)] struct Suffix { # [doc = " The starting position of this suffix."] # [doc = ""] # [doc = " If this is a forward suffix, then `&bytes[pos..]` can be used. If this"] # [doc = " is a reverse suffix, then `&bytes[..pos]` can be used. That is, for"] # [doc = " forward suffixes, this is an inclusive starting position, where as for"] # [doc = " reverse suffixes, this is an exclusive ending position."] pos : usize , # [doc = " The period of this suffix."] # [doc = ""] # [doc = " Note that this is NOT necessarily the period of the string from which"] # [doc = " this suffix comes from. (It is always less than or equal to the period"] # [doc = " of the original string.)"] period : usize , }
};
}
