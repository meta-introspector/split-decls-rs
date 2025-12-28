macro_rules! AttributeMatchGroup {
    () => {
        # [cfg (feature = "attributes")] type AttributeMatchGroup = gix_attributes :: Search ;
    };
}

AttributeMatchGroup!()