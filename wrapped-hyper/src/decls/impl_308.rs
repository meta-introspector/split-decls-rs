macro_rules! deps {
    () => {
        HeaderCaseMap!();
        OriginalHeaderOrder!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl Default for hyper_headers { fn default () -> Self { Self { headers : Default :: default () , orig_casing : HeaderCaseMap :: default () , orig_order : OriginalHeaderOrder :: default () , } } }
    };
}

impl_308!();