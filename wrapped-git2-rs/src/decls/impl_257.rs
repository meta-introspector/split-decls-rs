macro_rules! deps {
    () => {
        ParentIds!();
        Oid!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        # [doc = " Aborts iteration when a commit cannot be found"] impl < 'commit > Iterator for ParentIds < 'commit > { type Item = Oid ; fn next (& mut self) -> Option < Oid > { self . range . next () . and_then (| i | self . commit . parent_id (i) . ok ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_257!();