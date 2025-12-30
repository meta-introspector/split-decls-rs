// Generated macro for MultiUnzip (trait)
macro_rules! Depcrate_unziptupleMultiUnzip {
() => {
// Module: crate::unziptuple
// Provides: {"MultiUnzip"}
// Dependencies: {}
# [doc = " An iterator that can be unzipped into multiple collections."] # [doc = ""] # [doc = " See [`.multiunzip()`](crate::Itertools::multiunzip) for more information."] pub trait MultiUnzip < FromI > : Iterator { # [doc = " Unzip this iterator into multiple collections."] fn multiunzip (self) -> FromI ; }
};
}
