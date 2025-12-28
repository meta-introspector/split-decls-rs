macro_rules! ConsumerState {
    () => {
        # [doc = " The state of the consumer, used to communicate back to the source."] # [derive (Debug)] pub enum ConsumerState { # [doc = " The consumer is done making progress, and the `flush` method should be called."] Break , # [doc = " The consumer is ready to keep making progress."] Continue , # [doc = " The consumer currently holds no values and should not be called until"] # [doc = " more values have been provided to it."] Empty , }
    };
}

ConsumerState!();