macro_rules! RepeatErr {
    () => {
        # [doc = " An iterator that endlessly repeats a single error."] # [derive (Clone , Debug)] pub struct RepeatErr < T , E : Clone > (PhantomData < T > , E) ;
    };
}

RepeatErr!();