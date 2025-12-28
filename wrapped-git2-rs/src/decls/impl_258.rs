macro_rules! deps {
    () => {
        Oid!();
        ParentIds!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        # [doc = " Aborts iteration when a commit cannot be found"] impl < 'commit > DoubleEndedIterator for ParentIds < 'commit > { fn next_back (& mut self) -> Option < Oid > { self . range . next_back () . and_then (| i | self . commit . parent_id (i) . ok ()) } }
    };
}

impl_258!()