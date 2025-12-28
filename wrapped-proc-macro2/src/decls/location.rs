macro_rules! location {
    () => {
        # [cfg (span_locations)] mod location ;
    };
}

location!();