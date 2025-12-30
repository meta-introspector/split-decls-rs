// Generated macro for assign_fd (macro)
macro_rules! Depcrate_opcodeassign_fd {
() => {
// Module: crate::opcode
// Provides: {"assign_fd"}
// Dependencies: {}
macro_rules ! assign_fd { ($ sqe : ident . fd = $ opfd : expr) => { match $ opfd { sealed :: Target :: Fd (fd) => $ sqe . fd = fd , sealed :: Target :: Fixed (idx) => { $ sqe . fd = idx as _ ; $ sqe . flags |= crate :: squeue :: Flags :: FIXED_FILE . bits () ; } } } ; }
};
}
