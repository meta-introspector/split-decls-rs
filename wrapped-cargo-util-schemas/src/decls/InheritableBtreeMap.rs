macro_rules! deps {
    () => {
        InheritableField!();
    };
}

macro_rules! InheritableBtreeMap {
    () => {
        deps!();
        pub type InheritableBtreeMap = InheritableField < BTreeMap < String , BTreeMap < String , String > > > ;
    };
}

InheritableBtreeMap!()