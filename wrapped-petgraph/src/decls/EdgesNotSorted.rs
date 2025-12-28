macro_rules! deps {
    () => {
        Csr!();
    };
}

macro_rules! EdgesNotSorted {
    () => {
        deps!();
        # [doc = " Csr creation error: edges were not in sorted order."] # [derive (Clone , Debug)] pub struct EdgesNotSorted { # [allow (unused)] first_error : (usize , usize) , }
    };
}

EdgesNotSorted!();