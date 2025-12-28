macro_rules! deps {
    () => {
        IteratorItem!();
        Delimited!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < I : Iterator > Iterator for Delimited < I > { type Item = IteratorItem < I :: Item > ; fn next (& mut self) -> Option < Self :: Item > { let item = IteratorItem { value : self . iter . next () ? , is_first : self . is_first , is_last : self . iter . peek () . is_none () , } ; self . is_first = false ; Some (item) } }
    };
}

impl_54!()