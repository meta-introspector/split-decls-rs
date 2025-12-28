macro_rules! deps {
    () => {
        Attrs!();
    };
}

macro_rules! do_automock_once {
    () => {
        deps!();
        fn do_automock_once (attrs : TokenStream , input : TokenStream) -> TokenStream { let mut output = input . clone () ; let attrs : Attrs = match parse2 (attrs) { Ok (a) => a , Err (err) => { return err . to_compile_error () ; } } ; let item : Item = match parse2 (input) { Ok (item) => item , Err (err) => { return err . to_compile_error () ; } } ; output . extend (mock_it ((attrs , item))) ; output }
    };
}

do_automock_once!();