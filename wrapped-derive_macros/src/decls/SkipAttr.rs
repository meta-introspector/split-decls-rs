macro_rules! SkipAttr {
    () => {
        # [doc = "\nThe `skip` attribute.\n\nThis attribute signals that an item should be skipped\nfrom streaming.\n"] pub (crate) struct SkipAttr ;
    };
}

SkipAttr!();