macro_rules! informational {
    () => {
        # [cfg (all (feature = "http1" , feature = "client"))] mod informational ;
    };
}

informational!()