macro_rules! deps {
    () => {
        Name!();
        Error!();
    };
}

macro_rules! ExpandedTest {
    () => {
        deps!();
        struct ExpandedTest { name : Name , test : PathBuf , error : Option < Error > , }
    };
}

ExpandedTest!()