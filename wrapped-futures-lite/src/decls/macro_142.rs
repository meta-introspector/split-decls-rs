macro_rules! macro_142 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::step_by()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct StepBy < S > { # [pin] stream : S , step : usize , i : usize , } }
    };
}

macro_142!();