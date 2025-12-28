macro_rules! add {
    () => {
        fn add < T : Sum > (left : T , right : T) -> T { [left , right] . into_iter () . sum () }
    };
}

add!()