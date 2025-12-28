macro_rules! deps {
    () => {
        Table!();
        Segment!();
    };
}

macro_rules! Segments {
    () => {
        deps!();
        # [doc = " A segment table."] pub type Segments < 'data > = Table < Segment < 'data > > ;
    };
}

Segments!()