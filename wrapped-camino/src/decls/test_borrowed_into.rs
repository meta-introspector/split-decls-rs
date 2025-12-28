macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! test_borrowed_into {
    () => {
        deps!();
        # [test] fn test_borrowed_into () { let utf8_path = Utf8Path :: new ("test/path") ; all_into ! (& Utf8Path , utf8_path) ; }
    };
}

test_borrowed_into!();