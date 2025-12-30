// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "scrolling-regions")] impl crate :: crossterm :: Command for ScrollUpInRegion { fn write_ansi (& self , f : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { if self . lines_to_scroll != 0 { write ! (f , crate :: crossterm :: csi ! ("{};{}r") , self . first_row . saturating_add (1) , self . last_row . saturating_add (1)) ? ; write ! (f , crate :: crossterm :: csi ! ("{}S") , self . lines_to_scroll) ? ; write ! (f , crate :: crossterm :: csi ! ("r")) ? ; } Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { Err (io :: Error :: new (io :: ErrorKind :: Unsupported , "ScrollUpInRegion command not supported for winapi" ,)) } }
};
}
