macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! all_names {
    () => {
        deps!();
        pub fn all_names () -> Vec < & 'static str > { ExternAbi :: ALL_VARIANTS . iter () . map (| abi | abi . as_str ()) . collect () }
    };
}

all_names!()