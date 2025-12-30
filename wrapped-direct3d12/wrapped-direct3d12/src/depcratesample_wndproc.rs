// Generated macro for sample_wndproc (function)
macro_rules! Depcratesample_wndproc {
() => {
// Module: crate
// Provides: {"sample_wndproc"}
// Dependencies: {}
fn sample_wndproc < S : DXSample > (sample : & mut S , message : u32 , wparam : WPARAM) -> bool { match message { WM_KEYDOWN => { sample . on_key_down (wparam . 0 as u8) ; true } WM_KEYUP => { sample . on_key_up (wparam . 0 as u8) ; true } WM_PAINT => { sample . update () ; sample . render () ; true } _ => false , } }
};
}
