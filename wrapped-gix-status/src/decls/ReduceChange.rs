macro_rules! deps {
    () => {
        Entry!();
        VisitEntry!();
    };
}

macro_rules! ReduceChange {
    () => {
        deps!();
        struct ReduceChange < 'a , 'index , T : VisitEntry < 'index > > { collector : & 'a mut T , entries : & 'index [gix_index :: Entry] , }
    };
}

ReduceChange!()