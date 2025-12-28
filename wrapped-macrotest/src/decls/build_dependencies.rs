macro_rules! deps {
    () => {
        Result!();
        Error!();
        Project!();
    };
}

macro_rules! build_dependencies {
    () => {
        deps!();
        # [doc = " Builds dependencies for macro expansion and pipes `cargo` output to `STDOUT`."] # [doc = " Tries to expand macros in `main.rs` and intentionally filters the result."] # [doc = " This function is called before macro expansions to speed them up and"] # [doc = " for dependencies build process to be visible for user."] pub (crate) fn build_dependencies (project : & Project) -> Result < () > { use std :: io :: Write ; let stdout = cargo (project) . arg ("expand") . arg ("--bin") . arg (project . name . clone ()) . arg ("--theme") . arg ("none") . stdout (std :: process :: Stdio :: piped ()) . spawn () ? . stdout . ok_or (Error :: CargoFail) ? ; let reader = std :: io :: BufReader :: new (stdout) ; reader . lines () . filter_map (| line | line . ok ()) . filter (| line | ! line . starts_with ("fn main() {}")) . filter (| line | ! line_should_be_ignored (line)) . for_each (| line | { let _ = writeln ! (std :: io :: stdout () , "{}" , line) ; }) ; Ok (()) }
    };
}

build_dependencies!()