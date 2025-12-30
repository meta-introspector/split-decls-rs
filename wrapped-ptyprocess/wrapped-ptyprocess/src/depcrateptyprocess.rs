// Generated macro for PtyProcess (struct)
macro_rules! DepcratePtyProcess {
() => {
// Module: crate
// Provides: {"PtyProcess"}
// Dependencies: {}
# [doc = " PtyProcess controls a spawned process and communication with this."] # [doc = ""] # [doc = " It implements [std::io::Read] and [std::io::Write] to communicate with"] # [doc = " a child."] # [doc = ""] # [doc = " ```no_run,ignore"] # [doc = " use ptyprocess::PtyProcess;"] # [doc = " use std::io::Write;"] # [doc = " use std::process::Command;"] # [doc = ""] # [doc = " let mut process = PtyProcess::spawn(Command::new(\"cat\")).unwrap();"] # [doc = " process.write_all(b\"Hello World\").unwrap();"] # [doc = " process.flush().unwrap();"] # [doc = " ```"] # [derive (Debug)] pub struct PtyProcess { master : Master , child_pid : Pid , eof_char : u8 , intr_char : u8 , terminate_delay : Duration , }
};
}
