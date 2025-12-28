macro_rules! DeclRefactoringConfig {
    () => {
        # [derive (Deserialize , Default)] pub struct DeclRefactoringConfig { pub target_paths : Vec < String > , pub macro_crate_name : String , pub main_macro_lib_path : Option < String > , pub main_macro_invocation_modules : Vec < String > , }
    };
}

DeclRefactoringConfig!()