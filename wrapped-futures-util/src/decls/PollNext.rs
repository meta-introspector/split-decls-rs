macro_rules! PollNext {
    () => {
        # [doc = " Type to tell [`SelectWithStrategy`] which stream to poll next."] # [derive (Debug , PartialEq , Eq , Copy , Clone , Hash)] pub enum PollNext { # [doc = " Poll the first stream."] Left , # [doc = " Poll the second stream."] Right , }
    };
}

PollNext!()