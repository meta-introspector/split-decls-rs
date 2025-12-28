macro_rules! enable_autodiff_settings {
    () => {
        fn enable_autodiff_settings (ad : & [config :: AutoDiff]) { for val in ad { match val { config :: AutoDiff :: PrintPerf => { llvm :: set_print_perf (true) ; } config :: AutoDiff :: PrintAA => { llvm :: set_print_activity (true) ; } config :: AutoDiff :: PrintTA => { llvm :: set_print_type (true) ; } config :: AutoDiff :: PrintTAFn (fun) => { llvm :: set_print_type (true) ; llvm :: set_print_type_fun (& fun) ; } config :: AutoDiff :: Inline => { llvm :: set_inline (true) ; } config :: AutoDiff :: LooseTypes => { llvm :: set_loose_types (true) ; } config :: AutoDiff :: PrintSteps => { llvm :: set_print (true) ; } config :: AutoDiff :: PrintPasses => { } config :: AutoDiff :: PrintModBefore => { } config :: AutoDiff :: PrintModAfter => { } config :: AutoDiff :: PrintModFinal => { } config :: AutoDiff :: Enable => { } config :: AutoDiff :: NoPostopt => { } } } llvm :: set_strict_aliasing (false) ; llvm :: set_rust_rules (true) ; }
    };
}

enable_autodiff_settings!();