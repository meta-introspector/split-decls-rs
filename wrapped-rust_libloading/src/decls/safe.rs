macro_rules! safe {
    () => {
        # [cfg (any (unix , windows , libloading_docs))] mod safe ;
    };
}

safe!()