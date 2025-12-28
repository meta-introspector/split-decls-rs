macro_rules! deps {
    () => {
        CoffSegment!();
        CoffSegmentIterator!();
        Item!();
        CoffHeader!();
        ReadRef!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > Iterator for CoffSegmentIterator < 'data , 'file , R , Coff > { type Item = CoffSegment < 'data , 'file , R , Coff > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| section | CoffSegment { file : self . file , section , }) } }
    };
}

impl_209!();