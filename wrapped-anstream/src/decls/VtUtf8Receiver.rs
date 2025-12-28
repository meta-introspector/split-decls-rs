macro_rules! VtUtf8Receiver {
    () => {
        struct VtUtf8Receiver < 'a > (& 'a mut bool) ;
    };
}

VtUtf8Receiver!();