macro_rules! test {
    () => {
        # [cfg (all (test , feature = "alloc"))] mod test ;
    };
}

test!()