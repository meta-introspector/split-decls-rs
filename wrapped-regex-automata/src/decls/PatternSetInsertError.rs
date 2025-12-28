macro_rules! deps {
    () => {
        PatternID!();
        PatternSet!();
    };
}

macro_rules! PatternSetInsertError {
    () => {
        deps!();
        # [doc = " An error that occurs when a `PatternID` failed to insert into a"] # [doc = " `PatternSet`."] # [doc = ""] # [doc = " An insert fails when the given `PatternID` exceeds the configured capacity"] # [doc = " of the `PatternSet`."] # [doc = ""] # [doc = " This error is created by the [`PatternSet::try_insert`] routine."] # [cfg (feature = "alloc")] # [derive (Clone , Debug)] pub struct PatternSetInsertError { attempted : PatternID , capacity : usize , }
    };
}

PatternSetInsertError!();