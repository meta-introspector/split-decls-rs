// Generated macro for impl_38 (impl)
macro_rules! Depcrate_cursorimpl_38 {
() => {
// Module: crate::cursor
// Provides: {"impl_38"}
// Dependencies: {}
impl Command for SetCursorStyle { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { match self { SetCursorStyle :: DefaultUserShape => f . write_str ("\x1b[0 q") , SetCursorStyle :: BlinkingBlock => f . write_str ("\x1b[1 q") , SetCursorStyle :: SteadyBlock => f . write_str ("\x1b[2 q") , SetCursorStyle :: BlinkingUnderScore => f . write_str ("\x1b[3 q") , SetCursorStyle :: SteadyUnderScore => f . write_str ("\x1b[4 q") , SetCursorStyle :: BlinkingBar => f . write_str ("\x1b[5 q") , SetCursorStyle :: SteadyBar => f . write_str ("\x1b[6 q") , } } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
