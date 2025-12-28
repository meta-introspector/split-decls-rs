macro_rules! deps {
    () => {
        Point!();
        TypedArena!();
    };
}

macro_rules! test_unused {
    () => {
        deps!();
        # [test] fn test_unused () { let arena : TypedArena < Point > = TypedArena :: default () ; assert ! (arena . chunks . borrow () . is_empty ()) ; }
    };
}

test_unused!()