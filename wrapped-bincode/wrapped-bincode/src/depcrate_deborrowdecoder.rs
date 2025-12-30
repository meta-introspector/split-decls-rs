// Generated macro for BorrowDecoder (trait)
macro_rules! Depcrate_deBorrowDecoder {
() => {
// Module: crate::de
// Provides: {"BorrowDecoder"}
// Dependencies: {}
# [doc = " Any source that can decode basic types. This type is most notably implemented for [Decoder]."] # [doc = ""] # [doc = " This is an extension of [Decode] that can also return borrowed data."] pub trait BorrowDecoder < 'de > : Decoder { # [doc = " The concrete [BorrowReader] type"] type BR : BorrowReader < 'de > ; # [doc = " Returns a mutable reference to the borrow reader"] fn borrow_reader (& mut self) -> & mut Self :: BR ; }
};
}
