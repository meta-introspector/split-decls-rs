macro_rules! deps {
    () => {
        AtomicBool!();
    };
}

macro_rules! Notify {
    () => {
        deps!();
        # [doc = " Implements the park / unpark pattern directly using Loom's internal"] # [doc = " primitives."] # [doc = ""] # [doc = " Notification establishes an acquire / release synchronization point."] # [doc = " Only one thread may wait on each `Notify` at a time."] # [doc = ""] # [doc = " Using this type is useful to mock out constructs when using loom tests."] # [derive (Debug)] pub struct Notify { object : rt :: Notify , # [doc = " Enforces the single waiter invariant"] waiting : AtomicBool , }
    };
}

Notify!()