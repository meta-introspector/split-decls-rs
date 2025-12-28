macro_rules! deps {
    () => {
        RepeatErr!();
    };
}

macro_rules! repeat_err {
    () => {
        deps!();
        # [doc = " Creates an iterator that endlessly repeats a single error."] pub fn repeat_err < T , E : Clone > (value : E) -> RepeatErr < T , E > { RepeatErr (PhantomData , value) }
    };
}

repeat_err!()