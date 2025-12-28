macro_rules! DetectNonGenericPointeeAttr {
    () => {
        struct DetectNonGenericPointeeAttr < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , }
    };
}

DetectNonGenericPointeeAttr!();