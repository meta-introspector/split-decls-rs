macro_rules! deps {
    () => {
        Sender!();
        Token!();
        Receiver!();
    };
}

macro_rules! SelectedOperation {
    () => {
        deps!();
        # [doc = " A selected operation that needs to be completed."] # [doc = ""] # [doc = " To complete the operation, call [`send`] or [`recv`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Forgetting to complete the operation is an error and might lead to deadlocks. If a"] # [doc = " `SelectedOperation` is dropped without completion, a panic occurs."] # [doc = ""] # [doc = " [`send`]: SelectedOperation::send"] # [doc = " [`recv`]: SelectedOperation::recv"] # [must_use] pub struct SelectedOperation < 'a > { # [doc = " Token needed to complete the operation."] token : Token , # [doc = " The index of the selected operation."] index : usize , # [doc = " The address of the selected `Sender` or `Receiver`."] ptr : * const u8 , # [doc = " Indicates that `Sender`s and `Receiver`s are borrowed."] _marker : PhantomData < & 'a () > , }
    };
}

SelectedOperation!()