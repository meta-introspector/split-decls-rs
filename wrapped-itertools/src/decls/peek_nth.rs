macro_rules! peek_nth {
    () => {
        # [cfg (feature = "use_alloc")] mod peek_nth ;
    };
}

peek_nth!()