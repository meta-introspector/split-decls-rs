mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkitem!{cfg_select ! { target_os = "fuchsia" => { mod fuchsia ; use fuchsia as imp ; } target_os = "vxworks" => { mod vxworks ; use vxworks as imp ; } any (target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "nuttx") => { mod unsupported ; use unsupported as imp ; pub use unsupported :: output ; } _ => { mod unix ; use unix as imp ; } }}
mkuse!{pub use imp :: { ExitStatus , ExitStatusError , Process } ;}
mkuse!{pub use self :: common :: { Command , CommandArgs , ExitCode , Stdio , StdioPipes } ;}
mkuse!{pub use crate :: ffi :: OsString as EnvKey ;}