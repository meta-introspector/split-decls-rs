macro_rules! deps {
    () => {
        IndexType!();
        List!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl < E , Ix > fmt :: Debug for List < E , Ix > where E : fmt :: Debug , Ix : IndexType , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut fmt_struct = f . debug_struct ("adj::List") ; fmt_struct . field ("node_count" , & self . node_count ()) ; fmt_struct . field ("edge_count" , & self . edge_count ()) ; if self . edge_count () > 0 { fmt_struct . field ("edges" , & self . edge_references ()) ; } fmt_struct . finish () } }
    };
}

impl_299!()