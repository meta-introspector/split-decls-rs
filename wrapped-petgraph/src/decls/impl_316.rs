macro_rules! deps {
    () => {
        List!();
        IndexType!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < E , Ix : IndexType > NodeCount for List < E , Ix > { # [doc = " Returns the number of nodes in the list"] # [doc = ""] # [doc = " Computes in **O(1)** time."] fn node_count (& self) -> usize { self . suc . len () } }
    };
}

impl_316!();