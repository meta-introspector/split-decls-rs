macro_rules! macro_109 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::scan()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Scan < S , St , F > { # [pin] stream : S , state_f : (St , F) , } }
    };
}

macro_109!()