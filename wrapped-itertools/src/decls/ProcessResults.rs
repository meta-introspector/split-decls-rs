macro_rules! ProcessResults {
    () => {
        # [doc = " An iterator that produces only the `T` values as long as the"] # [doc = " inner iterator produces `Ok(T)`."] # [doc = ""] # [doc = " Used by [`process_results`](crate::process_results), see its docs"] # [doc = " for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug)] pub struct ProcessResults < 'a , I , E : 'a > { error : & 'a mut Result < () , E > , iter : I , }
    };
}

ProcessResults!();