// Minimal syscall module stub
// This is a placeholder to satisfy imports

pub struct SyscallTracker;

impl SyscallTracker {
    pub fn new() -> Self {
        Self
    }
}

pub fn init() -> SyscallTracker {
    SyscallTracker::new()
}
