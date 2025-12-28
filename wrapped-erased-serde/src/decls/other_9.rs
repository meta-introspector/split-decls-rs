macro_rules! other_9 {
    () => {
        union Value { ptr : * mut () , inline : [MaybeUninit < usize > ; 2] , }
    };
}

other_9!()