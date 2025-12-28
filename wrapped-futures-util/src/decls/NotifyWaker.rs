macro_rules! NotifyWaker {
    () => {
        struct NotifyWaker (task03 :: Waker) ;
    };
}

NotifyWaker!()