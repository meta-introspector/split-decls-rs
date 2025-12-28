macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! from_raw {
    () => {
        deps!();
        pub unsafe fn from_raw (raw : * mut curl_sys :: curl_slist) -> List { List { raw } }
    };
}

from_raw!();