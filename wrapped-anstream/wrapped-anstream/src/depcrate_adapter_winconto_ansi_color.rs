// Generated macro for to_ansi_color (function)
macro_rules! Depcrate_adapter_winconto_ansi_color {
() => {
// Module: crate::adapter::wincon
// Provides: {"to_ansi_color"}
// Dependencies: {}
fn to_ansi_color (digit : u16) -> Option < anstyle :: AnsiColor > { match digit { 0 => Some (anstyle :: AnsiColor :: Black) , 1 => Some (anstyle :: AnsiColor :: Red) , 2 => Some (anstyle :: AnsiColor :: Green) , 3 => Some (anstyle :: AnsiColor :: Yellow) , 4 => Some (anstyle :: AnsiColor :: Blue) , 5 => Some (anstyle :: AnsiColor :: Magenta) , 6 => Some (anstyle :: AnsiColor :: Cyan) , 7 => Some (anstyle :: AnsiColor :: White) , _ => None , } }
};
}
