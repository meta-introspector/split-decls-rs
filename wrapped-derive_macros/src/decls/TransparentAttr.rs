macro_rules! TransparentAttr {
    () => {
        # [doc = "\nThe `transparent` attribute.\n\nThis attribute signals that a newtype should stream its inner field\nwithout wrapping it in a tag.\n"] pub (crate) struct TransparentAttr ;
    };
}

TransparentAttr!()