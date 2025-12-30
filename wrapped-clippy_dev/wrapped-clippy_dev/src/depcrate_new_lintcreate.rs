// Generated macro for create (function)
macro_rules! Depcrate_new_lintcreate {
() => {
// Module: crate::new_lint
// Provides: {"create"}
// Dependencies: {}
# [doc = " Creates the files required to implement and test a new lint and runs `update_lints`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function errors out if the files couldn't be created or written to."] pub fn create (clippy_version : Version , pass : Pass , name : & str , category : & str , mut ty : Option < & str > , msrv : bool ,) -> io :: Result < () > { if category == "cargo" && ty . is_none () { ty = Some ("cargo") ; } let lint = LintData { clippy_version , pass , name , category , ty , } ; create_lint (& lint , msrv) . context ("Unable to create lint implementation") ? ; create_test (& lint , msrv) . context ("Unable to create a test for the new lint") ? ; if lint . ty . is_none () { add_lint (& lint , msrv) . context ("Unable to add lint to clippy_lints/src/lib.rs") ? ; } if pass == Pass :: Early { println ! ("\n\
            NOTE: Use a late pass unless you need something specific from\n\
            an early pass, as they lack many features and utilities") ; } Ok (()) }
};
}
