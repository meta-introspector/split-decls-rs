macro_rules! deps {
    () => {
        IndexType!();
        List!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < E , Ix : IndexType > EdgeCount for List < E , Ix > { # [doc = " Returns the number of edges in the list"] # [doc = ""] # [doc = " Computes in **O(|V|)** time."] fn edge_count (& self) -> usize { List :: edge_count (self) } }
    };
}

impl_317!();