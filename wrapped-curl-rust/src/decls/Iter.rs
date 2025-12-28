macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over `List`"] # [derive (Clone)] pub struct Iter < 'a > { _me : & 'a List , cur : * mut curl_sys :: curl_slist , }
    };
}

Iter!()