macro_rules! TDEFLFlush {
    () => {
        # [doc = " A list of deflate flush types."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum TDEFLFlush { # [doc = " Normal operation."] # [doc = ""] # [doc = " Compress as much as there is space for, and then return waiting for more input."] None = 0 , # [doc = " Try to flush all the current data and output an empty fixed block."] Partial = 1 , # [doc = " Try to flush all the current data and output an empty raw block."] Sync = 2 , # [doc = " Same as [`Sync`][Self::Sync], but reset the dictionary so that the following data does not"] # [doc = " depend on previous data."] Full = 3 , # [doc = " Try to flush everything and end the deflate stream."] # [doc = ""] # [doc = " On success this will yield a [`TDEFLStatus::Done`] return status."] Finish = 4 , }
    };
}

TDEFLFlush!()