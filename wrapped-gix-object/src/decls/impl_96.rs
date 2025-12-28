macro_rules! deps {
    () => {
        TagRefIter!();
        Error!();
        Token!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'a > Iterator for TagRefIter < 'a > { type Item = Result < Token < 'a > , crate :: decode :: Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . data . is_empty () { return None ; } match Self :: next_inner (self . data , & mut self . state) { Ok ((data , token)) => { self . data = data ; Some (Ok (token)) } Err (err) => { self . data = & [] ; Some (Err (err)) } } } }
    };
}

impl_96!();