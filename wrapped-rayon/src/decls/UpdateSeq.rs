macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! UpdateSeq {
    () => {
        deps!();
        # [doc = " Standard Update adaptor, based on `itertools::adaptors::Update`"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] struct UpdateSeq < I , F > { base : I , update_op : F , }
    };
}

UpdateSeq!();