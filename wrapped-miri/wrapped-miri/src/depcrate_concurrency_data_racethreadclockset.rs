// Generated macro for ThreadClockSet (struct)
macro_rules! Depcrate_concurrency_data_raceThreadClockSet {
() => {
// Module: crate::concurrency::data_race
// Provides: {"ThreadClockSet"}
// Dependencies: {}
# [doc = " The current set of vector clocks describing the state"] # [doc = " of a thread, contains the happens-before clock and"] # [doc = " additional metadata to model atomic fence operations."] # [derive (Clone , Default , Debug)] pub (super) struct ThreadClockSet { # [doc = " The increasing clock representing timestamps"] # [doc = " that happen-before this thread."] pub (super) clock : VClock , # [doc = " The set of timestamps that will happen-before this"] # [doc = " thread once it performs an acquire fence."] fence_acquire : VClock , # [doc = " The last timestamp of happens-before relations that"] # [doc = " have been released by this thread by a release fence."] fence_release : VClock , # [doc = " Timestamps of the last SC write performed by each"] # [doc = " thread, updated when this thread performs an SC fence."] # [doc = " This is never acquired into the thread's clock, it"] # [doc = " just limits which old writes can be seen in weak memory emulation."] pub (super) write_seqcst : VClock , # [doc = " Timestamps of the last SC fence performed by each"] # [doc = " thread, updated when this thread performs an SC read."] # [doc = " This is never acquired into the thread's clock, it"] # [doc = " just limits which old writes can be seen in weak memory emulation."] pub (super) read_seqcst : VClock , }
};
}
