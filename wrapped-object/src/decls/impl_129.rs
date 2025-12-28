macro_rules! deps {
    () => {
        Segment!();
        SegmentIteratorInternal!();
        SegmentIterator!();
        ReadRef!();
        Item!();
        SegmentInternal!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > Iterator for SegmentIterator < 'data , 'file , R > { type Item = Segment < 'data , 'file , R > ; fn next (& mut self) -> Option < Self :: Item > { next_inner ! (self . inner , SegmentIteratorInternal , SegmentInternal) . map (| inner | Segment { inner }) } }
    };
}

impl_129!()