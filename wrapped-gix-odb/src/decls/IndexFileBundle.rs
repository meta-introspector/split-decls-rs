macro_rules! deps {
    () => {
        OnDiskFile!();
    };
}

macro_rules! IndexFileBundle {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct IndexFileBundle { pub index : OnDiskFile < Arc < gix_pack :: index :: File > > , pub data : OnDiskFile < Arc < gix_pack :: data :: File > > , }
    };
}

IndexFileBundle!()