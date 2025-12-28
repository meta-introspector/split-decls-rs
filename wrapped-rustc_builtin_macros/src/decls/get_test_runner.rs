macro_rules! deps {
    () => {
        TestRunnerNargs!();
        Path!();
        TestRunnerInvalid!();
    };
}

macro_rules! get_test_runner {
    () => {
        deps!();
        fn get_test_runner (dcx : DiagCtxtHandle < '_ > , krate : & ast :: Crate) -> Option < ast :: Path > { let test_attr = attr :: find_by_name (& krate . attrs , sym :: test_runner) ? ; let meta_list = test_attr . meta_item_list () ? ; let span = test_attr . span ; match & * meta_list { [single] => match single . meta_item () { Some (meta_item) if meta_item . is_word () => return Some (meta_item . path . clone ()) , _ => { dcx . emit_err (errors :: TestRunnerInvalid { span }) ; } } , _ => { dcx . emit_err (errors :: TestRunnerNargs { span }) ; } } None }
    };
}

get_test_runner!();