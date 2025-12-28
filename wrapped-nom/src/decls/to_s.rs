macro_rules! to_s {
    () => {
        # [cfg (feature = "alloc")] fn to_s (i : Vec < u8 >) -> String { String :: from_utf8_lossy (& i) . into_owned () }
    };
}

to_s!()