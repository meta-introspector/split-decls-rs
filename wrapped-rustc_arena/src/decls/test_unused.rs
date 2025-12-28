macro_rules! deps {
    () => {
        TypedArena!();
        Point!();
    };
}

macro_rules! test_unused {
    () => {
        deps!();
        # [test] fn test_unused () { let arena : TypedArena < Point > = TypedArena :: default () ; assert ! (arena . chunks . borrow () . is_empty ()) ; }
    };
}

test_unused!();