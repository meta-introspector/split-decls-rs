use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FlushGuard {
    /// Signals the trace writing thread to flush to disk.
    pub fn flush(&self) {
        if let Some(handle) = self.handle.take() {
            let _ignored = self.sender.send(Message::Flush);
            self.handle.set(Some(handle));
        }
    }
    /// Finishes the current trace and starts a new one.
    ///
    /// If a [`Write`](std::io::Write) implementation is supplied,
    /// the new trace is written to it. Otherwise, the new trace
    /// goes to `./trace-{unix epoc in micros}.json`.
    pub fn start_new(&self, writer: Option<Box<dyn Write + Send>>) {
        if let Some(handle) = self.handle.take() {
            let _ignored = self.sender.send(Message::StartNew(writer));
            self.handle.set(Some(handle));
        }
    }
}
