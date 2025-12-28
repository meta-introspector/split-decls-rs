macro_rules! deps {
    () => {
        Segments!();
    };
}

macro_rules! SegmentId {
    () => {
        deps!();
        # [doc = " An ID for referring to a segment in [`Segments`]."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct SegmentId (usize) ;
    };
}

SegmentId!()