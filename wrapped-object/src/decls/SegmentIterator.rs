macro_rules! deps {
    () => {
        ReadRef!();
        SegmentIteratorInternal!();
        File!();
    };
}

macro_rules! SegmentIterator {
    () => {
        deps!();
        # [doc = " An iterator for the loadable segments in a [`File`]."] # [derive (Debug)] pub struct SegmentIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] > { inner : SegmentIteratorInternal < 'data , 'file , R > , }
    };
}

SegmentIterator!();