macro_rules! deps {
    () => {
        IndexRecord!();
    };
}

macro_rules! Index {
    () => {
        deps!();
        # [derive (Debug)] struct Index { pub number_of_records : u64 , pub records : Vec < IndexRecord > , }
    };
}

Index!()