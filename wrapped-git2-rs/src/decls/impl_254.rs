macro_rules! deps {
    () => {
        Commit!();
        Parents!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        # [doc = " Aborts iteration when a commit cannot be found"] impl < 'repo , 'commit > DoubleEndedIterator for Parents < 'commit , 'repo > { fn next_back (& mut self) -> Option < Commit < 'repo > > { self . range . next_back () . and_then (| i | self . commit . parent (i) . ok ()) } }
    };
}

impl_254!()