macro_rules! VisiblyUninhabited {
    () => {
        # [derive (PartialEq , Eq)] struct VisiblyUninhabited ;
    };
}

VisiblyUninhabited!();