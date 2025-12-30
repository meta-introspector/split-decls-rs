// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { use nes :: Interface ; benchlib :: benchmark :: run_benchmark_group (| group | { group . register_benchmark ("pinky-nes15" , | | { let mut nes = Emulator { state : nes :: State :: new () , } ; nes . load_rom (ROM) . unwrap () ; move | | { for _ in 0 .. 4 { nes . execute_until_vblank () . unwrap () ; } nes . press (nes :: ControllerPort :: First , nes :: Button :: Select) ; nes . execute_until_vblank () . unwrap () ; nes . release (nes :: ControllerPort :: First , nes :: Button :: Select) ; for _ in 0 .. 3 { nes . execute_until_vblank () . unwrap () ; } nes . press (nes :: ControllerPort :: First , nes :: Button :: Select) ; nes . execute_until_vblank () . unwrap () ; nes . release (nes :: ControllerPort :: First , nes :: Button :: Select) ; for _ in 0 .. 2 * 60 { nes . execute_until_vblank () . unwrap () ; } nes } }) ; }) ; }
};
}
