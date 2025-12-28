macro_rules! SingleOrMultiIndex {
    () => {
        pub (crate) enum SingleOrMultiIndex { Single { index : Arc < gix_pack :: index :: File > , data : Option < Arc < gix_pack :: data :: File > > , } , Multi { index : Arc < gix_pack :: multi_index :: File > , data : Vec < Option < Arc < gix_pack :: data :: File > > > , } , }
    };
}

SingleOrMultiIndex!();