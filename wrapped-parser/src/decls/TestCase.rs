macro_rules! TestCase {
    () => {
        # [derive (PartialEq , Eq , PartialOrd , Ord)] struct TestCase { rs : PathBuf , rast : PathBuf , text : String , }
    };
}

TestCase!()