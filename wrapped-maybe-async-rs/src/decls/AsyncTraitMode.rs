macro_rules! AsyncTraitMode {
    () => {
        enum AsyncTraitMode { Send , NotSend , Off , }
    };
}

AsyncTraitMode!()