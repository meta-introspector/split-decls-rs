macro_rules! deps {
    () => {
        EagerIter!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < I > Iterator for EagerIter < I > where I : Iterator + Send + 'static , < I as Iterator > :: Item : Send , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { match self . chunk . as_mut () { Some (chunk) => chunk . next () . or_else (| | self . fill_buf_and_pop ()) , None => self . fill_buf_and_pop () , } } fn size_hint (& self) -> (usize , Option < usize >) { self . size_hint } }
    };
}

impl_49!()