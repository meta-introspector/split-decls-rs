mkmod!{io_slice, { 
                getname!(io_slice);
                getsrc!(io_slice);
                getpath!(io_slice);
                get_deps!(io_slice);
                get_crates!(io_slice);
                mkinclude!(io_slice);
                mkitem!{cfg_select ! { any (target_family = "unix" , target_os = "hermit" , target_os = "solid_asp3" , target_os = "trusty") => { mod iovec ; pub use iovec ::*; } target_os = "windows" => { mod windows ; pub use windows ::*; } target_os = "wasi" => { mod wasi ; pub use wasi ::*; } target_os = "uefi" => { mod uefi ; pub use uefi ::*; } _ => { mod unsupported ; pub use unsupported ::*; } }} 
            }}
mkmod!{is_terminal, { 
                getname!(is_terminal);
                getsrc!(is_terminal);
                getpath!(is_terminal);
                get_deps!(is_terminal);
                get_crates!(is_terminal);
                mkinclude!(is_terminal);
                mkitem!{cfg_select ! { any (target_family = "unix" , target_os = "wasi") => { mod isatty ; pub use isatty ::*; } target_os = "windows" => { mod windows ; pub use windows ::*; } target_os = "hermit" => { mod hermit ; pub use hermit ::*; } _ => { mod unsupported ; pub use unsupported ::*; } }} 
            }}
mkuse!{pub use io_slice :: { IoSlice , IoSliceMut } ;}
mkuse!{pub use is_terminal :: is_terminal ;}
mkitem!{pub const DEFAULT_BUF_SIZE : usize = if cfg ! (target_os = "espidf") { 512 } else { 8 * 1024 } ;}