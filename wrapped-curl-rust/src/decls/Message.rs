macro_rules! deps {
    () => {
        Multi!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [doc = " Message from the `messages` function of a multi handle."] # [doc = ""] # [doc = " Currently only indicates whether a transfer is done."] pub struct Message < 'multi > { ptr : * mut curl_sys :: CURLMsg , _multi : & 'multi Multi , }
    };
}

Message!();