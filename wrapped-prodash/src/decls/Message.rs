macro_rules! deps {
    () => {
        MessageLevel!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [doc = " A message to be stored along with the progress tree."] # [doc = ""] # [doc = " It is created by [`Tree::message(…)`](./struct.Item.html#method.message)."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Message { # [doc = " The time at which the message was sent."] pub time : SystemTime , # [doc = " The severity of the message"] pub level : MessageLevel , # [doc = " The name of the task that created the `Message`"] pub origin : String , # [doc = " The message itself"] pub message : String , }
    };
}

Message!();