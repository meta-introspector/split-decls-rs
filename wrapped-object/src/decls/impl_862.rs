macro_rules! deps {
    () => {
        XcoffSegment!();
        XcoffSegmentIterator!();
        ReadRef!();
        Item!();
        FileHeader!();
    };
}

macro_rules! impl_862 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > Iterator for XcoffSegmentIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = XcoffSegment < 'data , 'file , Xcoff , R > ; fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_862!()