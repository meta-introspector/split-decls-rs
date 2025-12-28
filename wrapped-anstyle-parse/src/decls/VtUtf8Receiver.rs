macro_rules! VtUtf8Receiver {
    () => {
        # [cfg (feature = "utf8")] struct VtUtf8Receiver < 'a > (& 'a mut Option < char >) ;
    };
}

VtUtf8Receiver!()