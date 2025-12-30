// Generated macro for write_fmt (function)
macro_rules! Depcrate_winconwrite_fmt {
() => {
// Module: crate::wincon
// Provides: {"write_fmt"}
// Dependencies: {}
fn write_fmt (raw : & mut dyn anstyle_wincon :: WinconStream , state : & mut WinconBytes , args : std :: fmt :: Arguments < '_ > ,) -> std :: io :: Result < () > { let write_all = | buf : & [u8] | write_all (raw , state , buf) ; crate :: fmt :: Adapter :: new (write_all) . write_fmt (args) }
};
}
