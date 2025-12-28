macro_rules! deps {
    () => {
        MetadataType!();
        MetadataKindId!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl From < MetadataType > for MetadataKindId { fn from (value : MetadataType) -> Self { Self (value as c_uint) } }
    };
}

impl_488!();