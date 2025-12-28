macro_rules! RepeatWith {
    () => {
        # [doc = " Stream for the [`repeat_with()`] function."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct RepeatWith < F > { f : F , }
    };
}

RepeatWith!();