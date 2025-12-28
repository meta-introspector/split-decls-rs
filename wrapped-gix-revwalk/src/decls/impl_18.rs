macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'a , T > Index < & 'a gix_hash :: oid > for Graph < '_ , '_ , T > { type Output = T ; fn index (& self , index : & 'a oid) -> & Self :: Output { & self . map [index] } }
    };
}

impl_18!();