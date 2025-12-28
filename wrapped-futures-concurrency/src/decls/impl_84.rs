macro_rules! deps {
    () => {
        ReadinessVec!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl ReadinessVec { pub (crate) fn new () -> Self { Self { parent_waker : None } } # [doc = " Returns the old ready state for this id"] pub (crate) fn set_ready (& mut self , _id : usize) -> bool { false } # [doc = " Set all markers to ready."] pub (crate) fn set_all_ready (& mut self) { } # [doc = " Returns whether the task id was previously ready"] pub (crate) fn clear_ready (& mut self , _id : usize) -> bool { true } # [doc = " Returns `true` if any of the wakers are ready."] pub (crate) fn any_ready (& self) -> bool { true } # [doc = " Access the parent waker."] # [inline] pub (crate) fn parent_waker (& self) -> Option < & Waker > { self . parent_waker . as_ref () } # [doc = " Set the parent `Waker`. This needs to be called at the start of every"] # [doc = " `poll` function."] pub (crate) fn set_waker (& mut self , parent_waker : & Waker) { match & mut self . parent_waker { Some (prev) => prev . clone_from (parent_waker) , None => self . parent_waker = Some (parent_waker . clone ()) , } } # [doc = " Resize `readiness` to the new length."] # [doc = ""] # [doc = " If new entries are created, they will be marked as 'ready'."] pub (crate) fn resize (& mut self , _len : usize) { } }
    };
}

impl_84!()