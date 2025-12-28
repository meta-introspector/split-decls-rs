macro_rules! select_ok {
    () => {
        # [cfg (feature = "alloc")] mod select_ok ;
    };
}

select_ok!();