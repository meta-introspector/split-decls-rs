macro_rules! deps {
    () => {
        Error!();
        Name!();
    };
}

macro_rules! ExpandedTest {
    () => {
        deps!();
        struct ExpandedTest { name : Name , test : PathBuf , error : Option < Error > , }
    };
}

ExpandedTest!();