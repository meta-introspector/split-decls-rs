macro_rules! mul {
    () => {
        fn mul < T : Product > (left : T , right : T) -> T { [left , right] . into_iter () . product () }
    };
}

mul!();