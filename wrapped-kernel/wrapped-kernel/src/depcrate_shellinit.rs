// Generated macro for init (function)
macro_rules! Depcrate_shellinit {
() => {
// Module: crate::shell
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init () { let (print , read) = (| s : & str | { print ! ("{s}") ; crate :: console :: CONSOLE . lock () . flush () . unwrap () ; } , read ,) ; let mut shell = Shell :: new (print , read) ; shell . commands . insert ("help" , ShellCommand { help : "Print this help message" , func : | _ , shell | { shell . print_help_screen () ; Ok (()) } , aliases : & ["?" , "h"] , } ,) ; shell . commands . insert ("interrupts" , ShellCommand { help : "Shows the number of received interrupts" , func : | _ , _ | { print_statistics () ; Ok (()) } , aliases : & ["i"] , } ,) ; shell . commands . insert ("shutdown" , ShellCommand { help : "Shutdown HermitOS" , func : | _ , _ | crate :: scheduler :: shutdown (0) , aliases : & ["s"] , } ,) ; crate :: executor :: spawn (async move { shell . run_async () . await }) ; }
};
}
