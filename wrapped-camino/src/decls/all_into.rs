macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! all_into {
    () => {
        deps!();
        macro_rules ! all_into { ($ t : ty , $ x : ident) => { test_into ::<$ t , Utf8PathBuf > ($ x . clone ()) ; test_into ::<$ t , Box < Utf8Path >> ($ x . clone ()) ; test_into ::<$ t , Arc < Utf8Path >> ($ x . clone ()) ; test_into ::<$ t , Rc < Utf8Path >> ($ x . clone ()) ; test_into ::<$ t , Cow <'_ , Utf8Path >> ($ x . clone ()) ; test_into ::<$ t , PathBuf > ($ x . clone ()) ; test_into ::<$ t , Box < Path >> ($ x . clone ()) ; test_into ::<$ t , Arc < Path >> ($ x . clone ()) ; test_into ::<$ t , Rc < Path >> ($ x . clone ()) ; test_into ::<$ t , Cow <'_ , Path >> ($ x . clone ()) ; } ; }
    };
}

all_into!();