macro_rules! deps {
    () => {
        IndexItem!();
    };
}

macro_rules! Index {
    () => {
        deps!();
        # [derive (Default , Serialize)] struct Index { version : u8 , namespaces : Vec < String > , items : BTreeMap < usize , Vec < IndexItem > > , }
    };
}

Index!();