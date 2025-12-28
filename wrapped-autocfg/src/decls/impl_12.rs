macro_rules! deps {
    () => {
        Rustc!();
        Error!();
        Version!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Rustc { pub fn new () -> Self { Rustc { rustc : env :: var_os ("RUSTC") . unwrap_or_else (| | "rustc" . into ()) . into () , rustc_wrapper : get_rustc_wrapper (false) , rustc_workspace_wrapper : get_rustc_wrapper (true) , } } # [doc = " Build the command with possible wrappers."] pub fn command (& self) -> Command { let mut rustc = self . rustc_wrapper . iter () . chain (self . rustc_workspace_wrapper . iter ()) . chain (Some (& self . rustc)) ; let mut command = Command :: new (rustc . next () . unwrap ()) ; for arg in rustc { command . arg (arg) ; } command } # [doc = " Try to get the `rustc` version."] pub fn version (& self) -> Result < Version , Error > { macro_rules ! try_version { ($ command : expr) => { if let Ok (value) = Version :: from_command ($ command) { return Ok (value) ; } } ; } let rustc = & self . rustc ; if let Some (ref rw) = self . rustc_wrapper { if let Some (ref rww) = self . rustc_workspace_wrapper { try_version ! (Command :: new (rw) . args (& [rww , rustc])) ; } try_version ! (Command :: new (rw) . arg (rustc)) ; } if let Some (ref rww) = self . rustc_workspace_wrapper { try_version ! (Command :: new (rww) . arg (rustc)) ; } Version :: from_command (& mut Command :: new (rustc)) } }
    };
}

impl_12!()