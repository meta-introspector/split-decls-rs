macro_rules! test_vectors {
    () => {
        # [cfg (any (feature = "test-vectors" , test))] pub mod test_vectors ;
    };
}

test_vectors!()