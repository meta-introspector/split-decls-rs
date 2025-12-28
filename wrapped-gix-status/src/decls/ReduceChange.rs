macro_rules! deps {
    () => {
        VisitEntry!();
        Entry!();
    };
}

macro_rules! ReduceChange {
    () => {
        deps!();
        struct ReduceChange < 'a , 'index , T : VisitEntry < 'index > > { collector : & 'a mut T , entries : & 'index [gix_index :: Entry] , }
    };
}

ReduceChange!();