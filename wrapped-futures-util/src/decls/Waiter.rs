macro_rules! Waiter {
    () => {
        enum Waiter { Waiting (Waker) , Woken , }
    };
}

Waiter!()