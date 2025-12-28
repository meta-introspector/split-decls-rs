macro_rules! macro_119 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::then()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Then < S , F , Fut > { # [pin] stream : S , # [pin] future : Option < Fut >, f : F , } }
    };
}

macro_119!();