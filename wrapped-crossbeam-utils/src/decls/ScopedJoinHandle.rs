macro_rules! deps {
    () => {
        SharedOption!();
    };
}

macro_rules! ScopedJoinHandle {
    () => {
        deps!();
        # [doc = " A handle that can be used to join its scoped thread."] # [doc = ""] # [doc = " This struct is created by the [`Scope::spawn`] method and the"] # [doc = " [`ScopedThreadBuilder::spawn`] method."] pub struct ScopedJoinHandle < 'scope , T > { # [doc = " A join handle to the spawned thread."] handle : SharedOption < thread :: JoinHandle < () > > , # [doc = " Holds the result of the inner closure."] result : SharedOption < T > , # [doc = " A handle to the spawned thread."] thread : thread :: Thread , # [doc = " Borrows the parent scope with lifetime `'scope`."] _marker : PhantomData < & 'scope () > , }
    };
}

ScopedJoinHandle!()