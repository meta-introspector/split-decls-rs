// Generated macro for MaybeDelayed (enum)
macro_rules! Depcrate_driver_applyMaybeDelayed {
() => {
// Module: crate::driver::apply
// Provides: {"MaybeDelayed"}
// Dependencies: {}
# [doc = " A type to represent delayed or immediate apply-filter results."] pub enum MaybeDelayed < 'a > { # [doc = " Using the delayed protocol, this entry has been sent to a long-running process and needs to be"] # [doc = " checked for again, later, using the [`driver::Key`] to refer to the filter who owes a response."] # [doc = ""] # [doc = " Note that the path to the entry is also needed to obtain the filtered result later."] Delayed (driver :: Key) , # [doc = " The filtered result can be read from the contained reader right away."] # [doc = ""] # [doc = " Note that it must be consumed in full or till a read error occurs."] Immediate (Box < dyn std :: io :: Read + 'a >) , }
};
}
