macro_rules! deps {
    () => {
        PeSegmentIterator!();
        ReadRef!();
        Item!();
        ImageNtHeaders!();
        PeSegment!();
    };
}

macro_rules! impl_659 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > Iterator for PeSegmentIterator < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { type Item = PeSegment < 'data , 'file , Pe , R > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| section | PeSegment { file : self . file , section , }) } }
    };
}

impl_659!()