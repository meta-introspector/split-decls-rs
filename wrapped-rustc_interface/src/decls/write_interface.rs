macro_rules! write_interface {
    () => {
        pub fn write_interface < 'tcx > (tcx : TyCtxt < 'tcx >) { if ! tcx . crate_types () . contains (& rustc_session :: config :: CrateType :: Sdylib) { return ; } let _timer = tcx . sess . timer ("write_interface") ; let (_ , krate) = & * tcx . resolver_for_lowering () . borrow () ; let krate = rustc_ast_pretty :: pprust :: print_crate_as_interface (krate , tcx . sess . psess . edition , & tcx . sess . psess . attr_id_generator ,) ; let export_output = tcx . output_filenames (()) . interface_path () ; let mut file = fs :: File :: create_buffered (export_output) . unwrap () ; if let Err (err) = write ! (file , "{}" , krate) { tcx . dcx () . fatal (format ! ("error writing interface file: {}" , err)) ; } }
    };
}

write_interface!()