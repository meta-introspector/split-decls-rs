macro_rules! UnparkResult {
    () => {
        # [doc = " Result of an unpark operation."] # [derive (Copy , Clone , Default , Eq , PartialEq , Debug)] pub struct UnparkResult { # [doc = " The number of threads that were unparked."] pub unparked_threads : usize , # [doc = " The number of threads that were requeued."] pub requeued_threads : usize , # [doc = " Whether there are any threads remaining in the queue. This only returns"] # [doc = " true if a thread was unparked."] pub have_more_threads : bool , # [doc = " This is set to true on average once every 0.5ms for any given key. It"] # [doc = " should be used to switch to a fair unlocking mechanism for a particular"] # [doc = " unlock."] pub be_fair : bool , # [doc = " Private field so new fields can be added without breakage."] _sealed : () , }
    };
}

UnparkResult!();