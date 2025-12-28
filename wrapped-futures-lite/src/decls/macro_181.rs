macro_rules! macro_181 {
    () => {
        pin_project ! { # [doc = " Stream for the [`StreamExt::zip()`] method."] # [derive (Clone , Debug)] # [must_use = "streams do nothing unless polled"] pub struct Zip < A : Stream , B > { item_slot : Option < A :: Item >, # [pin] first : A , # [pin] second : B , } }
    };
}

macro_181!()