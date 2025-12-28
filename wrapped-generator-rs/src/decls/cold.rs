macro_rules! cold {
    () => {
        # [inline] # [cold] fn cold () { }
    };
}

cold!()