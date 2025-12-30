// Generated macro for CollationKeySink (trait)
macro_rules! Depcrate_comparisonCollationKeySink {
() => {
// Module: crate::comparison
// Provides: {"CollationKeySink"}
// Dependencies: {}
# [doc = " A [`std::io::Write`]-like trait for writing to a buffer-like object."] # [doc = ""] # [doc = " (This crate does not have access to [`std`].)"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement or call methods on this trait"] # [doc = " unless you are prepared for things to occasionally break."] # [doc = ""] # [doc = " Graduation tracking issue: [issue #7178](https://github.com/unicode-org/icu4x/issues/7178)."] # [doc = " </div>"] # [doc = ""] # [doc = " ✨ *Enabled with the `unstable` Cargo feature.*"] pub trait CollationKeySink { # [doc = " The type of error the sink may return."] type Error ; # [doc = " An intermediate state object used by the sink, which must implement [`Default`]."] type State ; # [doc = " A result value indicating the final state of the sink (e.g. a number of bytes written)."] type Output ; # [doc = " Writes a buffer into the writer."] fn write (& mut self , state : & mut Self :: State , buf : & [u8]) -> Result < () , Self :: Error > ; # [doc = " Write a single byte into the writer."] fn write_byte (& mut self , state : & mut Self :: State , b : u8) -> Result < () , Self :: Error > { self . write (state , & [b]) } # [doc = " Finalize any internal sink state (perhaps by flushing a buffer) and return the final"] # [doc = " output value."] fn finish (& mut self , state : Self :: State) -> Result < Self :: Output , Self :: Error > ; }
};
}
