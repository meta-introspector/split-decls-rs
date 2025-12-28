macro_rules! deps {
    () => {
        OnDiskFile!();
    };
}

macro_rules! MultiIndexFileBundle {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct MultiIndexFileBundle { pub multi_index : OnDiskFile < Arc < gix_pack :: multi_index :: File > > , pub data : Vec < OnDiskFile < Arc < gix_pack :: data :: File > > > , }
    };
}

MultiIndexFileBundle!()