macro_rules! List {
    () => {
        # [doc = " A linked list of a strings"] pub struct List { raw : * mut curl_sys :: curl_slist , }
    };
}

List!();