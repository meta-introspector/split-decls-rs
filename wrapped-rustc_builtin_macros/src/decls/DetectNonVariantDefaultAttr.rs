macro_rules! DetectNonVariantDefaultAttr {
    () => {
        struct DetectNonVariantDefaultAttr < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , }
    };
}

DetectNonVariantDefaultAttr!();