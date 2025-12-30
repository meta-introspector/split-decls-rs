// Generated macro for BlockReason (enum)
macro_rules! Depcrate_concurrency_threadBlockReason {
() => {
// Module: crate::concurrency::thread
// Provides: {"BlockReason"}
// Dependencies: {}
# [doc = " Keeps track of what the thread is blocked on."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum BlockReason { # [doc = " The thread tried to join the specified thread and is blocked until that"] # [doc = " thread terminates."] Join (ThreadId) , # [doc = " Waiting for time to pass."] Sleep , # [doc = " Blocked on a mutex."] Mutex , # [doc = " Blocked on a condition variable."] Condvar , # [doc = " Blocked on a reader-writer lock."] RwLock , # [doc = " Blocked on a Futex variable."] Futex , # [doc = " Blocked on an InitOnce."] InitOnce , # [doc = " Blocked on epoll."] Epoll , # [doc = " Blocked on eventfd."] Eventfd , # [doc = " Blocked on unnamed_socket."] UnnamedSocket , }
};
}
