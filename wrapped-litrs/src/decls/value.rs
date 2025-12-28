macro_rules! deps {
    () => {
        BoolLit!();
    };
}

macro_rules! value {
    () => {
        deps!();
        # [test] fn value () { assert ! (! BoolLit :: False . value ()) ; assert ! (BoolLit :: True . value ()) ; }
    };
}

value!()