macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! macro_46 {
    () => {
        deps!();
        if_hyper ! { pub (crate) fn try_uri (url : & Url) -> crate :: Result < http :: Uri > { url . as_str () . parse () . map_err (| _ | crate :: error :: url_invalid_uri (url . clone ())) } }
    };
}

macro_46!()