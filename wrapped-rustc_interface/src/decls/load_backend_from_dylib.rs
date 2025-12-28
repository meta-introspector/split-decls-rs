macro_rules! deps {
    () => {
        MakeBackendFn!();
    };
}

macro_rules! load_backend_from_dylib {
    () => {
        deps!();
        # [allow (rustc :: untranslatable_diagnostic)] fn load_backend_from_dylib (early_dcx : & EarlyDiagCtxt , path : & Path) -> MakeBackendFn { match unsafe { load_symbol_from_dylib :: < MakeBackendFn > (path , "__rustc_codegen_backend") } { Ok (backend_sym) => backend_sym , Err (DylibError :: DlOpen (path , err)) => { let err = format ! ("couldn't load codegen backend {path}{err}") ; early_dcx . early_fatal (err) ; } Err (DylibError :: DlSym (_path , err)) => { let e = format ! ("`__rustc_codegen_backend` symbol lookup in the codegen backend failed{err}" ,) ; early_dcx . early_fatal (e) ; } } }
    };
}

load_backend_from_dylib!();