// Generated macro for Parser (trait)
macro_rules! Depcrate_segParser {
() => {
// Module: crate::seg
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " A parser for incoming segments"] pub trait Parser : Default { # [doc = " The type of item that is parsed"] type Item : ? Sized ; # [doc = " The parsing error that may occur"] type Error ; # [doc = " The main parsing function"] # [doc = ""] # [doc = " This function processes the incoming bytes and returns the item."] # [doc = ""] # [doc = " One important detail that **MUST NOT** be overlooked is that the"] # [doc = " parser may save data from a previous parsing attempt. The number of"] # [doc = " bytes saved is indicated by the `Parser::saved()` function. The saved"] # [doc = " bytes will be copied into the beginning of the `bytes` array before"] # [doc = " processing. Therefore, two requirements should be met."] # [doc = ""] # [doc = " First, the incoming byte slice should be larger than the saved bytes."] # [doc = ""] # [doc = " Second, the incoming byte slice should contain new bytes only after"] # [doc = " the saved byte prefix."] # [doc = ""] # [doc = " If both criteria are met, this allows the parser to prepend its saved"] # [doc = " bytes without any additional allocation."] fn parse < 'a > (& mut self , bytes : & 'a mut [u8]) -> Result < & 'a Self :: Item , Self :: Error > ; # [doc = " Indicates the number of saved bytes in the parser"] fn saved (& self) -> usize { 0 } }
};
}
