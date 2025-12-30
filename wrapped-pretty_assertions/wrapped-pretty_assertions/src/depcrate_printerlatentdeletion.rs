// Generated macro for LatentDeletion (struct)
macro_rules! Depcrate_printerLatentDeletion {
() => {
// Module: crate::printer
// Provides: {"LatentDeletion"}
// Dependencies: {}
# [doc = " Delay formatting this deleted chunk until later."] # [doc = ""] # [doc = " It can be formatted as a whole chunk by calling `flush`, or the inner value"] # [doc = " obtained with `take` for further processing (such as an inline diff)."] # [derive (Default)] struct LatentDeletion < 'a > { value : Option < & 'a str > , count : usize , }
};
}
