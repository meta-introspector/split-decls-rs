macro_rules! deps {
    () => {
        EntryRef!();
        Error!();
        TreeRefIter!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'a > Iterator for TreeRefIter < 'a > { type Item = Result < EntryRef < 'a > , crate :: decode :: Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . data . is_empty () { return None ; } match decode :: fast_entry (self . data) { Some ((data_left , entry)) => { self . data = data_left ; Some (Ok (entry)) } None => { let failing = self . data ; self . data = & [] ; # [allow (clippy :: unit_arg)] Some (Err (crate :: decode :: Error :: with_err (winnow :: error :: ErrMode :: from_input (& failing) , failing ,))) } } } }
    };
}

impl_118!()