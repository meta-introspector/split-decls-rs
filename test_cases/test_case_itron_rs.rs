// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/sync/condvar/itron.rs
// Error: expected square brackets
// Problematic line: line 15

// The implementation is inspired by the queue-based implementation shown in
// Andrew D. Birrell's paper "Implementing Condition Variables with Semaphores"

pub struct Condvar {
    waiters: SpinMutex<waiter_queue::WaiterQueue>,
}

