macro_rules! groupbylazy {
    () => {
        # [cfg (feature = "use_alloc")] mod groupbylazy ;
    };
}

groupbylazy!()