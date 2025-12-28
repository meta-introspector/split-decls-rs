macro_rules! Version {
    () => {
        # [doc = " Version information about libcurl and the capabilities that it supports."] pub struct Version { inner : * mut curl_sys :: curl_version_info_data , }
    };
}

Version!();