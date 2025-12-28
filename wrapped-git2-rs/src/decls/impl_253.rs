macro_rules! deps {
    () => {
        Parents!();
        Commit!();
    };
}

macro_rules! impl_253 {
    () => {
        deps!();
        # [doc = " Aborts iteration when a commit cannot be found"] impl < 'repo , 'commit > Iterator for Parents < 'commit , 'repo > { type Item = Commit < 'repo > ; fn next (& mut self) -> Option < Commit < 'repo > > { self . range . next () . and_then (| i | self . commit . parent (i) . ok ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_253!();