// Generated macro for impl_197 (impl)
macro_rules! Depcrate_executorimpl_197 {
() => {
// Module: crate::executor
// Provides: {"impl_197"}
// Dependencies: {}
impl CaptureKind { fn for_config (config : & Config) -> Self { if config . nocapture { Self :: None } else if config . new_output_capture { Self :: New { buf : output_capture :: CaptureBuf :: new () } } else { Self :: Old { buf : Default :: default () } } } fn should_set_panic_hook (& self) -> bool { match self { Self :: None => false , Self :: Old { .. } => true , Self :: New { .. } => true , } } fn stdout (& self) -> & dyn ConsoleOut { self . capture_buf_or (& output_capture :: Stdout) } fn stderr (& self) -> & dyn ConsoleOut { self . capture_buf_or (& output_capture :: Stderr) } fn capture_buf_or < 'a > (& 'a self , fallback : & 'a dyn ConsoleOut) -> & 'a dyn ConsoleOut { match self { Self :: None | Self :: Old { .. } => fallback , Self :: New { buf } => buf , } } fn into_inner (self) -> Option < Vec < u8 > > { match self { Self :: None => None , Self :: Old { buf } => Some (buf . lock () . unwrap_or_else (| e | e . into_inner ()) . to_vec ()) , Self :: New { buf } => Some (buf . into_inner () . into ()) , } } }
};
}
