// Generated macro for ask_to_run (function)
macro_rules! Depcrate_utilask_to_run {
() => {
// Module: crate::util
// Provides: {"ask_to_run"}
// Dependencies: {}
pub fn ask_to_run (mut cmd : Command , ask : bool , text : & str) { let is_ci = env :: var_os ("CI") . is_some () || env :: var_os ("TF_BUILD") . is_some () ; if ask && ! is_ci { let mut buf = String :: new () ; print ! ("I will run `{cmd:?}` to {text}. Proceed? [Y/n] ") ; io :: stdout () . flush () . unwrap () ; io :: stdin () . read_line (& mut buf) . unwrap () ; match buf . trim () . to_lowercase () . as_ref () { "" | "y" | "yes" => { } "n" | "no" => show_error ! ("aborting as per your request") , a => show_error ! ("invalid answer `{}`" , a) , } ; } else { eprintln ! ("Running `{cmd:?}` to {text}.") ; } if cmd . status () . unwrap_or_else (| _ | panic ! ("failed to execute {cmd:?}")) . success () . not () { show_error ! ("failed to {}" , text) ; } }
};
}
