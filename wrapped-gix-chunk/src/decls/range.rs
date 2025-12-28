macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! range {
    () => {
        deps!();
        # [doc = ""] pub mod range { use std :: ops :: Range ; use crate :: file ; # [doc = " Turn a u64 Range into a usize range safely, to make chunk ranges useful in memory mapped files."] pub fn into_usize (Range { start , end } : Range < file :: Offset >) -> Option < Range < usize > > { let start = start . try_into () . ok () ? ; let end = end . try_into () . ok () ? ; Some (Range { start , end }) } # [doc = " Similar to [`into_usize()`], but panics assuming that the memory map couldn't be created if offsets"] # [doc = " stored are too high."] # [doc = ""] # [doc = " This is only true for correctly formed files, as it's entirely possible to provide out of bounds offsets"] # [doc = " which are checked for separately - we wouldn't be here if that was the case."] pub fn into_usize_or_panic (range : Range < file :: Offset >) -> Range < usize > { into_usize (range) . expect ("memory maps can't be created if files are too large") } }
    };
}

range!();