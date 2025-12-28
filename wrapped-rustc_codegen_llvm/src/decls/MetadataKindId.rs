macro_rules! MetadataKindId {
    () => {
        # [derive (Copy , Clone)] # [repr (transparent)] pub (crate) struct MetadataKindId (c_uint) ;
    };
}

MetadataKindId!()