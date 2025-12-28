macro_rules! RepeatWith {
    () => {
        # [doc = " An stream that repeats elements of type `A` endlessly by"] # [doc = " applying the provided closure `F: FnMut() -> A`."] # [doc = ""] # [doc = " This `struct` is created by the [`repeat_with()`] function."] # [doc = " See its documentation for more."] # [derive (Debug , Clone)] # [must_use = "streams do nothing unless polled"] pub struct RepeatWith < F > { repeater : F , }
    };
}

RepeatWith!()