// Generated macro for DXSample (trait)
macro_rules! DepcrateDXSample {
() => {
// Module: crate
// Provides: {"DXSample"}
// Dependencies: {}
trait DXSample { fn new (command_line : & SampleCommandLine) -> Result < Self > where Self : Sized ; fn bind_to_window (& mut self , hwnd : & HWND) -> Result < () > ; fn update (& mut self) { } fn render (& mut self) { } fn on_key_up (& mut self , _key : u8) { } fn on_key_down (& mut self , _key : u8) { } fn title (& self) -> String { "DXSample" . into () } fn window_size (& self) -> (i32 , i32) { (640 , 480) } }
};
}
