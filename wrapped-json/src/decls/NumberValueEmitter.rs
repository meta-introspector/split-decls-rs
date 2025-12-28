macro_rules! NumberValueEmitter {
    () => {
        # [cfg (feature = "arbitrary_precision")] struct NumberValueEmitter ;
    };
}

NumberValueEmitter!()