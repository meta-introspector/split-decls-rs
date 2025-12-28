macro_rules! deps {
    () => {
        OffsetIter!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a , T > Iterator for OffsetIter < 'a , T > { type Item = (usize , & 'a [T]) ; fn next (& mut self) -> Option < Self :: Item > { let block = self . inner . next () ? ; let offset = self . offset ; self . offset += block . len () ; Some ((offset , block)) } }
    };
}

impl_26!()