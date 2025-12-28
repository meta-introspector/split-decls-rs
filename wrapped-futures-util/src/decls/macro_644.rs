macro_rules! deps {
    () => {
        FlattenUnordered!();
    };
}

macro_rules! macro_644 {
    () => {
        deps!();
        pin_project ! { # [doc = " Emits either successful streams or single-item streams containing the underlying errors."] # [doc = " This's a wrapper for `FlattenUnordered` to reuse its logic over `TryStream`."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct NestedTryStreamIntoEitherTryStream < St > where St : TryStream , St :: Ok : TryStream , St :: Ok : Unpin , < St :: Ok as TryStream >:: Error : From < St :: Error > { # [pin] stream : St } }
    };
}

macro_644!();