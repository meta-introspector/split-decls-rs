macro_rules! EnumerateFuture {
    () => {
        # [doc = " Takes a future and maps it to another future via a closure"] # [derive (Debug)] # [pin_project :: pin_project] pub struct EnumerateFuture < FutT , T > where FutT : Future < Output = T > , { done : bool , # [pin] fut_t : FutT , count : usize , }
    };
}

EnumerateFuture!();