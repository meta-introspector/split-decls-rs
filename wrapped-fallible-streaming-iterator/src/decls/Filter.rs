macro_rules! Filter {
    () => {
        # [doc = " An iterator which filters elements with a predicate."] pub struct Filter < I , F > { it : I , f : F , }
    };
}

Filter!()