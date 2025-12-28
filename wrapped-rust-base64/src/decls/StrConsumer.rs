macro_rules! StrConsumer {
    () => {
        # [doc = " An abstraction around consuming `str`s produced by base64 encoding."] pub trait StrConsumer { # [doc = " Consume the base64 encoded data in `buf`"] fn consume (& mut self , buf : & str) ; }
    };
}

StrConsumer!();