macro_rules! com {
    () => {
        # [cfg (windows)] mod com ;
    };
}

com!()